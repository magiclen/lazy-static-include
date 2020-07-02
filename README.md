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

lazy_static_include_str! {
    /// doc
    TEST => "data/test.txt",
    /// doc
    pub TEST2 => "data/test-2.txt",
}

assert_eq!("This is just a test text.", TEST);
assert_eq!(TEST2, "Some text...");
```

```rust
#[macro_use] extern crate lazy_static_include;

lazy_static_include_bytes! {
    /// doc
    TEST => "data/test.txt",
    /// doc
    pub TEST2 => "data/test-2.txt",
}

assert_eq!(b"This is just a test text.".as_ref(), TEST);
assert_eq!(TEST2, b"Some text...".as_ref());
```

You should notice that the value created from `lazy_static_include_bytes` and `lazy_static_include_str` macros isn't equal to `&'static [u8]` or `&'static str` when you are not using the **release** profile. If you want to get an exact `&'static [u8]` or `&'static str` reference, you can **dereference** the value or just use the `as_ref` method.

```rust
#[macro_use] extern crate lazy_static_include;

lazy_static_include_bytes! {
    /// doc
    TEST => "data/test.txt",
}

#[cfg(debug_assertions)]
let data: &'static [u8] = *TEST;

#[cfg(not(debug_assertions))]
let data: &'static [u8] = TEST;

let data: &'static [u8] = TEST.as_ref();
```

## Include Array

There is a special macro `lazy_static_include_array` which can include arrays from files.
The array is fixed sized and can be one of these following types: `bool`, `char`, `usize`, `u8`, `u16`, `u32`, `u64`, `u128`, `isize`, `i8`, `i16`, `i32`, `i64`, `i128`, `f32`, `f64`, `&'static str`.

Also, the `lazy_static_include_array` macro includes data from files into the compiled executable binary file **only** when you are using the **release** profile.
Be careful when you distribute your program.

The paths used for `lazy_static_include_array` are relative to **CARGO_MANIFEST_DIR**.

```rust
#[macro_use] extern crate lazy_static_include;

lazy_static_include_array! {
    /// doc
    TEST: [u64; 5] => "data/u64_array.txt",
    /// doc
    pub TEST2: [&'static str; 3] => "data/string_array.txt"
}

assert_eq!(123, TEST[0]);
assert_eq!(456, TEST[1]);
assert_eq!(789, TEST[2]);
assert_eq!(1000, TEST[3]);
assert_eq!(500000000000u64, TEST[4]);

assert_eq!("Hi", TEST2[0]);
assert_eq!("Hello", TEST2[1]);
assert_eq!("哈囉", TEST2[2]);
```

## Benchmark

Using static mechanisms makes your program faster. See my benchmark result below (AMD Ryzen 9 3900X 12-Core Processor 12C/24T 3.90GHz, ran on 2020/07/02):

```text
test include_array_lazy_static   ... bench:          45 ns/iter (+/- 3)
test include_array_native_static ... bench:          45 ns/iter (+/- 3)
test include_array_no_static     ... bench:      20,959 ns/iter (+/- 295)
test include_bytes_lazy_static   ... bench:         754 ns/iter (+/- 7)
test include_bytes_native_static ... bench:         755 ns/iter (+/- 11)
test include_bytes_no_static     ... bench:       4,560 ns/iter (+/- 179)
test include_str_lazy_static     ... bench:         753 ns/iter (+/- 10)
test include_str_native_static   ... bench:         755 ns/iter (+/- 7)
test include_str_no_static       ... bench:       4,830 ns/iter (+/- 198)
```

When using the **release** profile, the performance of `lazy_static_include_*` is very close to `include_*` (in fast, they are the same). That means you don't need to worry about the overhead, but just enjoy the faster compilation time.

You can run the benchmark program by executing,

```bash
cargo bench
```

## Crates.io

https://crates.io/crates/lazy-static-include

## Documentation

https://docs.rs/lazy-static-include

## License

[MIT](LICENSE)