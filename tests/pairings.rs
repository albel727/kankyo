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