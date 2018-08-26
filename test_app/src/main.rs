extern crate kankyo;

use std::env;
use std::io::Cursor;

// Test that a line that starts with a comment is not put in the sys::env, and
// that trying to retrieve it errors.
#[test]
fn test_commented_line() {
    kankyo::load().unwrap();

    env::var("#commented line").is_err();
}

// Test that attempting to load the default '.env' file by the default
// 'kankyo' function works and does not error.
#[test]
fn test_default_location() {
    kankyo::load().unwrap();
}

// Test that all of the keys in the .env file were successfully put into the
// environment, except for the commented line which is tested in another
// test.
//
// These should all be strings and should not auto-cast to their types
// (e.g. a 1 should be "1" and not a 1i32).
#[test]
fn test_envs_set() {
    kankyo::load().unwrap();

    assert_eq!(env::var("test").unwrap(), "a");
    assert_eq!(env::var("test2").unwrap(), "1");
    assert_eq!(env::var("TESTKEY").unwrap(), "TESTVAL");
    assert_eq!(env::var("TEST_KEY").unwrap(), "test_val");
    assert!(env::var("NONEXISTENT").is_err());
}

// Test that loading a key from the environment results an an Ok. This is
// equivilant to using the environment itself, but is shorter or one less use
// statement.
#[test]
fn test_key() {
    // Tests that a 'cleaner' way to get a key returns a correct value.
    kankyo::load().unwrap();

    assert!(kankyo::key("test").is_some());
    assert!(kankyo::key("does_not_exist").is_none());
}

// Test that a snapshot is successfully produced and has the correct key-value
// pairs.
#[test]
fn test_snapshot() {
    kankyo::unload().unwrap();

    let before = kankyo::snapshot();

    kankyo::load().unwrap();
    let after = kankyo::snapshot();
    assert!(!before.contains_key("test"));
    assert!(after.contains_key("test"));
}

// Test that unloading all of the keys from the default '.env' file works.
#[test]
fn test_unload() {
    assert!(kankyo::load().is_ok());
    assert!(kankyo::key("test").is_some());
    assert!(kankyo::unload().is_ok());
    assert!(kankyo::key("test").is_none());
}

#[test]
fn test_unload_from_reader() {
    let mut lines = Cursor::new("FOO=bar\nBAR=baz".to_owned().into_bytes());

    kankyo::load_from_reader(&mut lines).unwrap();
    assert!(kankyo::key("FOO").is_some());

    // reset the cursor
    lines.set_position(0);

    kankyo::unload_from_reader(&mut lines).unwrap();
    assert!(kankyo::key("FOO").is_none()); // should no longer exist
}
