extern crate kankyo;

use kankyo::*;
use std::io::Cursor;

#[test]
fn test_key() {
    utils::set_variables(&[("foo", "1")]);
    assert!(key("foo").is_some());
    utils::unload(&["foo"]);
}

#[test]
fn test_loaders() {
    load().unwrap();
    unload().unwrap();
}

#[test]
fn test_reader_loaders() {
    let mut cursor = Cursor::new(b"A=B\nC=D");

    load_from_reader(&mut cursor).unwrap();

    cursor.set_position(0);
    unload_from_reader(&mut cursor).unwrap();
}
