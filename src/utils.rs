//! A collection of lower-level functions for parsing .env files, unloading
//! keys, and working with parsed .env lines.
//!
//! You most likely do not need to work with these yourself and should usually
//! prefer the higher-level functions in the root module of the library.

use std::env;

/// A key-value pair of a line from a .env file.
///
/// # Examples
///
/// In the instance of a line with the content `"FOO=bar"`, after being parsed
/// tuple index 0 will be `"FOO"` and index 1 will be `"bar"`.
///
/// ```rust
/// use kankyo::utils;
///
/// let line = utils::parse_line("FOO=bar").unwrap();
/// assert_eq!(line.0, "FOO");
/// assert_eq!(line.1, "bar");
/// ```
pub type ParsedLine<'a> = (&'a str, &'a str);

/// Maps the given slice of [`ParsedLine`] into a vector of their keys.
///
/// # Examples
///
/// ```rust
/// use kankyo::utils;
///
/// let buf = "FOO=bar\nBAR=baz";
/// let lines = utils::parse_lines(buf);
/// let mut keys = vec![];
/// utils::only_keys(&lines, &mut keys);
///
/// assert_eq!(keys, vec!["FOO", "BAR"]);
/// ```
///
/// [`ParesedLines`]: type.ParsedLine.html
// This accepts a mutable reference to a Vec so that, if the user already has
// one to use, they can pass it instead of us creating a new one.
//
// i.e.: scenario where this will _slighty_ improve performance.
pub fn only_keys<'a>(lines: &'a [ParsedLine], keys: &mut Vec<&'a str>) {
    for &(key, _) in lines {
        keys.push(key);
    }
}

/// Returns a `Vec` of `ParsedLine`s, each line representing a parsed K-V of the
/// given file.
pub fn parse_lines(buf: &str) -> Vec<ParsedLine> {
    buf.lines().filter_map(parse_line).collect()
}

/// Parses a .env file line.
///
/// This will take a line and return a tuple of the key and value, where the
/// tuple values map to the string `"$0=$1"`.
///
/// In the event the input string does not match the above format, `None` will
/// be returned.
///
/// # Examples
///
/// Assert that parsing various strings either properly parse or do not:
///
/// ```rust
/// use kankyo::utils;
///
/// assert!(utils::parse_line("hello").is_none()); // does not properly parse
/// assert!(utils::parse_line("HELLO=world").is_some()); // does parse
/// assert!(utils::parse_line("HELLO=world=!").is_some());
/// assert!(utils::parse_line("HELLO   =world!").is_some());
/// assert!(utils::parse_line("HELLO=").is_some()); // a 0-length value is valid
/// ```
pub fn parse_line(line: &str) -> Option<ParsedLine> {
    let (equals, comment) = (line.find('='), line.find('#'));

    if let Some(comment) = comment {
        if let Some(equals) = equals {
            if comment < equals {
                return None;
            }
        }
    }

    equals.map(|pos_equals| {
        // We have the position of the equals sign, so we know for sure what the
        // key is.
        let key = &line[..pos_equals];
        // We skip the equals sign, so add one to the position..
        let post_idx = pos_equals + 1;
        // Slice the input line and check if there's a hash.
        //
        // The hash is where a comment, if there is one, begins.
        //
        // If there is a hash, then slice from `post_idx` until its position. If
        // there is not, slice from `post_idx` until the end.
        let value = comment
            .map(|pos_pound| &line[post_idx..pos_pound])
            .unwrap_or_else(|| &line[post_idx..]);

        (key.trim(), value.trim())
    })
}

/// Loads the given slice of parsed lines into the environment.
///
/// # Examples
///
/// ```rust
/// use kankyo::utils;
///
/// let content = "FOO=bar\nBAR=baz";
///
/// let lines = utils::parse_lines(content);
///
/// utils::set_variables(&lines);
/// ```
pub fn set_variables(lines: &[ParsedLine]) {
    for line in lines {
        if env::var(line.0).is_ok() {
            continue;
        }

        env::set_var(line.0, line.1);
    }
}

/// Unloads the given slice of keys from the environment.
///
/// This effectively iterates over the given slice and calls `env::remove_var`
/// on each.
///
/// # Examples
///
/// ```rust,no_run
/// use kankyo::utils;
///
/// utils::unload(&["FOO", "BAR"]);
/// ```
pub fn unload(keys: &[&str]) {
    for key in keys {
        env::remove_var(key);
    }
}
