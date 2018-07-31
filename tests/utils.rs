extern crate kankyo;

use kankyo::utils::*;
use std::env;

#[test]
fn test_set_variables() {
    set_variables(&[("foo", "1"), ("bar", "2")]);
    assert!(env::var("foo").is_ok());

    unload(&["foo"]);
    assert!(env::var("bar").is_ok());
    unload(&["bar"]);
    assert!(env::var("bar").is_err());
}

#[test]
fn test_unload() {
    env::set_var("foo", "1");
    assert!(env::var("foo").is_ok());

    unload(&["foo"]);
    assert!(env::var("foo").is_err());
}

#[test]
fn test_unload_from_parsed_lines() {
    env::set_var("foo", "1");
    assert!(env::var("foo").is_ok());

    unload_from_parsed_lines(&[("foo", "1")]);
    assert!(env::var("foo").is_err());
}
