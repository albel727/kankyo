//! kankyo
//!
//! `kankyo` is a crate for the loading and unloading of `.env` files or other
//! readers into and from the environment.
//!
//! This crate is meant to be a more modular and efficient, yet concise
//! collection of functions exposed for any custom requirement. Due to its
//! design, it is applicable in both synchronous and asynchronous applications.
//!
//! ### Installation
//!
//! Add the following dependency to your project's `Cargo.toml`:
//!
//! ```toml
//! kankyo = "~0.1"
//! ```
//!
//! ### What are `.env` files?
//!
//! Environment variable files, often named `.env`, are files usually located at
//! the project root. The contents of the file are `=` (equals sign)-delimited
//! lines of key-value pairs. A sample file might look like:
//!
//! ```ini
//! DEBUG=info
//! DB_HOST=127.0.0.1 # This is a comment, not parsed as part of the value.
//!
//! # Empty lines are ignored, as are lines solely consisting of a comment.
//! ```
//!
//! The library works with interfacing over readers (types implementing the
//! `std::io::Read` trait), meaning that you can pass slices of bytes, strings,
//! files, etc. to it.
//!
//! For example, opening a file and parsing its contents into the environment:
//!
//! ```rust,no_run
//! extern crate kankyo;
//!
//! use std::fs::File;
//!
//! # use std::error::Error;
//! #
//! # fn try_main() -> Result<(), Box<Error>> {
//! kankyo::load_from_reader(&mut File::open("./.env")?)?;
//!
//! println!("Loaded!");
//! #     Ok(())
//! # }
//! #
//! # fn main() {
//! #     try_main().unwrap();
//! # }
//! ```
//!
//! Due to the common nature of this operation, a function that does precisely
//! this is offered:
//!
//! ```rust,no_run
//! extern crate kankyo;
//!
//! # use std::error::Error;
//! #
//! # fn try_main() -> Result<(), Box<Error>> {
//! kankyo::load()?;
//!
//! println!("Loaded!");
//! #     Ok(())
//! # }
//! #
//! # fn main() {
//! #     try_main().unwrap();
//! # }
//! ```
#![deny(missing_docs)]

pub mod utils;

mod error;
mod internal;

pub use error::Result;

use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Loads a key from the current environment. This is more or less an alias of
/// `std::env::var`, but the benefit - slightly - is one less possible use
/// statement.
///
/// # Examples
///
/// Retrieve a key from the environment:
///
/// ```rust,no_run
/// # use std::error::Error;
/// #
/// # fn try_main() -> Result<(), Box<Error>> {
/// kankyo::load()?;
///
/// if let Some(value) = kankyo::key("MY_KEY") {
///     println!("The value of MY_KEY is: {}", value);
/// }
/// #     Ok(())
/// # }
/// #
/// # fn main() {
/// #     try_main().unwrap();
/// # }
/// ```
#[inline]
pub fn key(name: &str) -> Option<String> {
    env::var(name).ok()
}

/// Loads a `.env` file at the current working directory (`./.env`).
///
/// # Examples
///
/// ```rust,no_run
/// # use std::error::Error;
/// #
/// # fn try_main() -> Result<(), Box<Error>> {
/// kankyo::load()?;
///
/// println!("Loaded!");
/// #     Ok(())
/// # }
/// #
/// # fn main() {
/// #     try_main().unwrap();
/// # }
/// ```
///
/// # Errors
///
/// Returns an `std::io::Error` if there was an error reading the file.
#[inline]
pub fn load() -> Result<()> {
    load_from_reader(&mut File::open(Path::new(".env"))?)
}

/// Reads the content of a reader and parses it to find `.env` lines.
///
/// # Errors
///
/// Returns an `std::io::Error` if there was an error reading from the reader.
pub fn load_from_reader<R: Read>(reader: &mut R) -> Result<()> {
    let content = internal::read_to_string(reader)?;
    utils::set_variables(&utils::parse_lines(&content));

    Ok(())
}

/// Creates a snapshot of the present environment variables.
///
/// This is similar to `std::env::vars`, but will instead return a HashMap over
/// only the environment variables that are valid UTF-8.
///
/// # Examples
///
/// ```rust,no_run
/// # use std::error::Error;
/// #
/// # fn try_main() -> Result<(), Box<Error>> {
/// let snapshot = kankyo::snapshot();
///
/// kankyo::load()?;
///
/// #     Ok(())
/// }
/// #
/// # fn main() {
/// #     try_main().unwrap();
/// # }
/// ```
pub fn snapshot() -> HashMap<String, String> {
    env::vars_os().into_iter().filter_map(|(key, value)| {
        if let (Ok(k), Ok(v)) = (key.into_string(), value.into_string()) {
            Some((k, v))
        } else {
            None
        }
    }).collect()
}

/// Unloads all environment variables in the default `./.env` file from the
/// current environment.
///
/// # Examples
///
/// ```rust,no_run
/// # use std::error::Error;
/// #
/// # fn try_main() -> Result<(), Box<Error>> {
/// kankyo::load()?;
/// println!("Loaded!");
///
/// kankyo::unload()?;
/// println!("Unloaded!");
/// #     Ok(())
/// # }
/// #
/// # fn main() {
/// #     try_main().unwrap();
/// # }
/// ```
///
/// # Errors
///
/// Returns an `std::io::Error` if there was an error reading from the reader.
#[inline]
pub fn unload() -> Result<()> {
    unload_from_reader(&mut File::open(".env")?)
}

/// Unloads from the read content of the given reader.
///
/// The reader should contain content that of a `.env` file.
///
/// If you need to unload a given slice of keys, prefer [`utils::unload`].
///
/// # Examples
///
/// Unload from a file at the path `./.env`:
///
/// ```rust,no_run
/// # use std::error::Error;
/// #
/// # fn try_main() -> Result<(), Box<Error>> {
/// #
/// use std::fs::File;
///
/// kankyo::unload_from_reader(&mut File::open("./.env")?)?;
/// println!("Successfully unloaded from `./.env`");
/// #     Ok(())
/// # }
/// #
/// # fn main() {
/// #     try_main().unwrap();
/// # }
/// ```
///
/// # Errors
///
/// Returns an `std::io::Error` if there is an error reading from the reader.
///
/// [`utils::unload`]: utils/fn.unload.html
pub fn unload_from_reader<R: Read>(reader: &mut R) -> Result<()> {
    let buf = internal::read_to_string(reader)?;
    let lines = utils::parse_lines(&buf);
    utils::unload_from_parsed_lines(&lines);

    Ok(())
}
