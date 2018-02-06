extern crate kankyo;

use kankyo::utils::*;
use std::ffi::OsString;
use std::env;

#[test]
fn test_parse_kv() {
    let mut key = OsString::new();
    key.push("A");
    let mut value = OsString::new();
    value.push("B");
    assert!(parse_kv((key, value.clone())).is_some());

    let mut modify = String::new();
    modify.push('\x7f');

    unsafe {
        modify.as_mut_vec()[0] += 1;
    }

    let invalid_key = OsString::from(modify);

    assert!(parse_kv((invalid_key, value)).is_none());
}

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
