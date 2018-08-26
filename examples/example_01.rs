extern crate kankyo;

use std::env;
use std::io::Cursor;

fn main() {
    // You should almost always load your environment variables at the very
    // beginning of your binary.

    // We'll be using the reader function. Normally you would read from a file
    // or some other reader, but we'll just use a Cursor for simplicity.
    let mut cursor = Cursor::new("FOO=bar\nBAR=baz".to_owned().into_bytes());

    // Check that "FOO" isn't an environment variable:
    println!("FOO doesn't exist: {}", env::var("FOO").is_err());

    // Set the environment variables from the cursor.
    //
    // This if-let is only required in Rust 1.3 and below, as `Result::expect`
    // was not released until Rust 1.4.
    if let Err(why) = kankyo::load_from_reader(&mut cursor) {
        panic!("Err loading: {:?}", why);
    }

    // Print the "FOO" environment variable to verify it now exists:
    println!("FOO exists: {}", env::var("FOO").is_ok());

    // Now to unload the environment variables:
    cursor.set_position(0);

    if let Err(why) = kankyo::unload_from_reader(&mut cursor) {
        panic!("Err unloading: {:?}", why);
    }

    // Verify "FOO" isn't an environment variable:
    println!("FOO doesn't exist: {}", env::var("FOO").is_err());
}
