[![ci-badge][]][ci]

`kankyo` is a crate for the loading and unloading of `.env` files or other
readers into and from the environment.

This crate is meant to be a more modular and efficient, yet concise
collection of functions exposed for any custom requirement. Due to its
design, it is applicable in both synchronous and asynchronous applications.

### Installation

Add the following dependency to your project's `Cargo.toml`:

```toml
kankyo = "~0.2"
```

### What are `.env` files?

Environment variable files, often named `.env`, are files usually located at
the project root. The contents of the file are `=` (equals sign)-delimited
lines of key-value pairs. A sample file might look like:

```ini
DEBUG=info
DB_HOST=127.0.0.1 # This is a comment, not parsed as part of the value.

# Empty lines are ignored, as are lines solely consisting of a comment.
```

### Usage

The library works with interfacing over readers (types implementing the
`std::io::Read` trait), meaning that you can pass slices of bytes, strings,
files, etc. to it.

For example, opening a file and parsing its contents into the environment:

```rust,no_run
extern crate kankyo;

use std::fs::File;

kankyo::load_from_reader(File::open("./.env")?)?;

println!("Loaded!");
```

Due to the common nature of this operation, a function that does precisely
this is offered:

```rust,no_run
extern crate kankyo;

kankyo::load()?;

println!("Loaded!");
```

### License

License info in [LICENSE.md]. Long story short, ISC.

[ci]: https://travis-ci.org/zeyla/kankyo
[ci-badge]: https://travis-ci.org/zeyla/kankyo.svg?branch=master
[LICENSE.md]: https://github.com/zeyla/kankyo/blob/master/LICENSE.md
