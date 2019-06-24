//! A collection of lower-level functions for parsing .env files, unloading
//! keys, and working with parsed .env lines.
//!
//! You most likely do not need to work with these yourself and should usually
//! prefer the higher-level functions in the [root module] of the library.
//!
//! [root module]: ../index.html

use std::ffi::OsString;
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
/// Parse a string of the variable pairs `("FOO", "bar")` and `("BAR", "baz")`
/// into `ParsedLine`s and then retrieve only the keys:
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
/// [`ParsedLine`]: type.ParsedLine.html
// This accepts a mutable reference to a Vec so that, if the user already has
// one to use, they can pass it instead of us creating a new one.
//
// reason: there are scenarios where this will _slighty_ improve performance,
// perhaps if the user has a vector of keys they are already working with.
pub fn only_keys<'a>(lines: &'a [ParsedLine], keys: &mut Vec<&'a str>) {
    for &(key, _) in lines {
        keys.push(key);
    }
}

/// Returns a `Vec` of `ParsedLine`s, each line representing a parsed key-value
/// pair of the given buffer.
///
/// # Examples
///
/// Parses a buffer into parsed lines, which are a key-value pair of string
/// slices:
///
/// ```rust
/// use kankyo::utils;
///
/// let input = "FOO=bar\nBAZ=qux";
/// let lines = utils::parse_lines(input);
///
/// // Make sure there are two pairs in the resultant vector:
/// assert_eq!(lines.len(), 2);
/// ```
#[inline]
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

    if let (Some(comment), Some(equals)) = (comment, equals) {
        if comment < equals {
            return None;
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

/// Parses a K-V pair of an environment variable OsString name and value into
/// their String equivalents.
pub fn parse_kv(pair: (OsString, OsString)) -> Option<(String, String)> {
    if let (Ok(k), Ok(v)) = (pair.0.into_string(), pair.1.into_string()) {
        Some((k, v))
    } else {
        None
    }
}

/// Loads the given slice of parsed lines into the environment.
///
/// Additionally you can pass whether to overwrite existing variables with the
/// same name.
///
/// # Examples
///
/// Parse a buffer into parsed lines, and then load the lines into the
/// environment:
///
/// ```rust
/// use kankyo::utils;
///
/// let content = "FOO=bar\nBAR=baz";
///
/// let lines = utils::parse_lines(content);
///
/// utils::set_variables(&lines, true);
/// ```
pub fn set_variables(lines: &[ParsedLine], overwrite: bool) {
    for line in lines {
        if !overwrite && env::var(line.0).is_ok() {
            continue;
        }

        env::set_var(line.0, line.1);
    }
}

/// Unloads the given slice of keys from the environment.
///
/// This effectively iterates over the given slice and calls `env::remove_var`
/// on each value.
///
/// # Examples
///
/// Unload the environment variables `"FOO"` and `"BAR"`:
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

/// Unloads from the given borrowed [`ParsedLine`]s.
///
/// This is going to be slightly more efficient than taking the result of
/// [`parse_lines`], passing it through [`only_keys`], and then finally passing
/// it through [`unload`], as it does not go through the extra step of mapping
/// `ParsedLine`s 0-element tuple values to a Vec.
///
/// # Examples
///
/// Unload the environment variables `"KEY"` and `"KEY2"` after parsing the
/// input string:
///
/// ```rust,no_run
/// use kankyo::utils;
///
/// let string = "KEY=VALUE\nKEY2=VALUE2\n# a comment";
/// let lines = utils::parse_lines(string);
///
/// // Ensure both keys are present for unloading:
/// assert_eq!(lines[0].0, "KEY");
/// assert_eq!(lines[1].0, "VALUE");
///
/// utils::unload_from_parsed_lines(&lines);
/// ```
///
/// [`ParsedLine`]: type.ParsedLine.html
/// [`only_keys`]: fn.only_keys.html
/// [`parse_lines`]: fn.parse_lines.html
/// [`unload`]: fn.unload.html
pub fn unload_from_parsed_lines(lines: &[ParsedLine]) {
    for &(key, _) in lines {
        env::remove_var(key);
    }
}

#[cfg(test)]
mod test {
    use std::ffi::OsString;
    use utils;

    #[test]
    fn pairings() {
        assert_eq!(utils::parse_line("key=value"), Some(("key", "value")));
        assert_eq!(utils::parse_line("key =value"), Some(("key", "value")));
        assert_eq!(utils::parse_line(" key ="), Some(("key", "")));
    }

    #[test]
    fn comments() {
        assert_eq!(utils::parse_line("key#=value"), None);
        assert_eq!(utils::parse_line("key=#abc"), Some(("key", "")));
    }

    #[test]
    fn only_keys() {
        let lines = utils::parse_lines("KEY=value\nKEY2=value2");
        let mut vec = Vec::with_capacity(lines.len());
        utils::only_keys(&lines, &mut vec);

        assert_eq!(vec, &["KEY", "KEY2"]);
    }

    #[test]
    fn parse_line() {
        assert_eq!(utils::parse_line("KEY=value"), Some(("KEY", "value")));
        assert_eq!(utils::parse_line("KEY=value#test"), Some(("KEY", "value")));
        assert!(utils::parse_line("KEY").is_none());
        assert_eq!(utils::parse_line("KEY="), Some(("KEY", "")));
        assert!(utils::parse_line("KEY#B=C#").is_none());
    }

    #[test]
    fn parse_lines() {
        assert_eq!(utils::parse_lines("A=B\nC=D\nE=F#").len(), 3);
    }

    #[test]
    fn test_parse_kv() {
        let mut key = OsString::new();
        key.push("A");
        let mut value = OsString::new();
        value.push("B");
        assert!(utils::parse_kv((key, value.clone())).is_some());

        let mut modify = String::new();
        modify.push('\x7f');

        unsafe {
            modify.as_mut_vec()[0] += 1;
        }

        let invalid_key = OsString::from(modify);

        assert!(utils::parse_kv((invalid_key, value)).is_none());
    }
}
