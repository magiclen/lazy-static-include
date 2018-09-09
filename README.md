Lazy Static Include
====================

[![Build Status](https://travis-ci.org/magiclen/lazy-static-include.svg?branch=master)](https://travis-ci.org/magiclen/lazy-static-include)

This crate provides `lazy_static_include_bytes` and `lazy_static_include_str` macros to replace `include_bytes` and `include_str` macros.

Why should we do that?
Because the original `include_bytes` and `include_str` macros bring extra data from files into the compiled executable binary file, the time for compiling surges.

High compilation time is detrimental to software development. `lazy_static_include_bytes` and `lazy_static_include_str` macros can help you **lazy load** data from files
when you are not using the **release** profile. In other words, if you are using `include_bytes` and `include_str` macros, and you think your compilation time is too high to wait,
you can choose to use `lazy_static_include_bytes` and `lazy_static_include_str` macros.

`lazy_static_include_bytes` and `lazy_static_include_str` macros include data from files into the compiled executable binary file **only** when you are using the **release** profile.
Be careful when you distribute your program.

The paths used for `lazy_static_include_bytes` and `lazy_static_include_str` are relative to **CARGO_MANIFEST_DIR**.

## Examples

```rust
#[macro_use] extern crate lazy_static_include;
#[macro_use] extern crate lazy_static;

lazy_static_include_str!(TEST, "data/test.txt");
lazy_static_include_str!(pub TEST2, "data/test-2.txt");

assert_eq!("This is just a test text.", TEST);
assert_eq!(TEST2, "Some text...");
```

```rust
#[macro_use] extern crate lazy_static_include;
#[macro_use] extern crate lazy_static;

lazy_static_include_bytes!(TEST, "data/test.txt", "data/test-2.txt");

assert_eq!("This is just a test text.".as_bytes(), TEST[0]);
assert_eq!(TEST[1], "Some text...".as_bytes());
```

You should notice that the struct created from `include_bytes` and `include_str` macros isn't equal to `&'static [u8]` or `&'static str`.
If you want to get an exact `&'static [u8]` or `&'static str` reference, you need to **dereference the struct**.

```rust
#[macro_use] extern crate lazy_static_include;
#[macro_use] extern crate lazy_static;

lazy_static_include_bytes!(TEST, "data/test.txt");

let data: &'static [u8] = *TEST;
```

If you include str and bytes from multiple files, after dereferencing the struct, you will get a `Vec<&'static [u8]>` or a `Vec<&'static str>`.
In order to not move out of borrowed content, use **&*** to get the reference of that `Vec`.

```rust
#[macro_use] extern crate lazy_static_include;
#[macro_use] extern crate lazy_static;

lazy_static_include_str!(TEST, "data/test.txt", "data/test-2.txt");

let v: &Vec<&'static str> = &*TEST;
```

## Crates.io

https://crates.io/crates/lazy-static-include

## Documentation

https://docs.rs/lazy-static-include

## License

[MIT](LICENSE)