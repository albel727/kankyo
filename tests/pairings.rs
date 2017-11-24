//! Assert that certain key-value pairings are parsed correctly.
//!
//! For example, `KEY =VALUE` should parse as `KEY` and `VALUE`, not `KEY ` and
//! `VALUE`.

extern crate kankyo;

use kankyo::utils::*;
use kankyo::*;

#[test]
fn pairings() {
    assert_eq!(parse_line("key=value"), Some(("key", "value")));
    assert_eq!(parse_line("key =value"), Some(("key", "value")));
    assert_eq!(parse_line(" key ="), Some(("key", "")));
}
