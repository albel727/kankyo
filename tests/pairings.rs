//! Assert that certain key-value pairings are parsed correctly.
//!
//! For example, `KEY =VALUE` should parse as `KEY` and `VALUE`, not `KEY ` and
//! `VALUE`.

extern crate kankyo;

use kankyo::*;

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
