Lazy Static Include
====================

[![Build Status](https://travis-ci.org/magiclen/lazy-static-include.svg?branch=master)](https://travis-ci.org/magiclen/lazy-static-include)
[![Build status](https://ci.appveyor.com/api/projects/status/gvc26sm6ytnucmn2/branch/master?svg=true)](https://ci.appveyor.com/project/magiclen/lazy-static-include/branch/master)

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

You should notice that the struct created from `lazy_static_include_bytes` and `lazy_static_include_str` macros isn't equal to `&'static [u8]` or `&'static str`.
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

## Include Array

There is a special macro `lazy_static_include_array` which can include arrays from files.
The array is fixed sized and can be one of these following types: `bool`, `char`, `usize`, `u8`, `u16`, `u32`, `u64`, `u128`, `isize`, `i8`, `i16`, `i32`, `i64`, `i128`, `f32`, `f64`, `&'static str`.

Also, the `lazy_static_include_array` macro includes data from files into the compiled executable binary file **only** when you are using the **release** profile.
Be careful when you distribute your program.

```rust
#[macro_use] extern crate lazy_static_include;
#[macro_use] extern crate lazy_static;

lazy_static_include_array!(TEST: [u64; 5], "data/u64_array.txt");
assert_eq!(123, TEST[0]);
assert_eq!(456, TEST[1]);
assert_eq!(789, TEST[2]);
assert_eq!(1000, TEST[3]);
assert_eq!(500000000000u64, TEST[4]);
```

```rust
#[macro_use] extern crate lazy_static_include;
#[macro_use] extern crate lazy_static;

lazy_static_include_array!(TEST: [i32; 5], "data/i32_array.txt", "data/i32_array-2.txt");
assert_eq!(123, TEST[0][0]);
assert_eq!(-456, TEST[0][1]);
assert_eq!(789, TEST[0][2]);
assert_eq!(1000, TEST[0][3]);
assert_eq!(5000, TEST[0][4]);

assert_eq!(-1, TEST[1][0]);
assert_eq!(-2, TEST[1][1]);
assert_eq!(-3, TEST[1][2]);
assert_eq!(-4, TEST[1][3]);
assert_eq!(-5, TEST[1][4]);
```

```rust
#[macro_use] extern crate lazy_static_include;
#[macro_use] extern crate lazy_static;

lazy_static_include_array!(pub TEST: [&'static str; 3], "data/string_array.txt");

assert_eq!("Hi", TEST[0]);
assert_eq!("Hello", TEST[1]);
assert_eq!("哈囉", TEST[2]);
```

## No Std

This crate can work without std, but the `lazy_static_include_array` macro will be disabled unless using the **release** profile.

Enable the feature **no_std** to compile this crate without std.

```toml
[dependencies.lazy-static-include]
version = "*"
features = ["no_std"]
default-features = false
```

## Benchmark

Using static mechanisms makes your program faster. See my benchmark result below (Intel i7-6700HQ, ran on 2018/11/14):

```text
include_str/include_str_no_static
                        time:   [8.3773 us 8.4061 us 8.4361 us]
include_str/include_str_native_static
                        time:   [965.65 ns 969.47 ns 973.04 ns]
include_str/include_str_lazy_static
                        time:   [955.93 ns 958.78 ns 961.88 ns]

include_bytes/include_bytes_no_static
                        time:   [7.7806 us 7.8056 us 7.8318 us]
include_bytes/include_bytes_native_static
                        time:   [418.43 ns 420.12 ns 421.83 ns]
include_bytes/include_bytes_lazy_static
                        time:   [413.43 ns 415.14 ns 417.37 ns]

include_array/include_array_no_static
                        time:   [30.125 us 30.285 us 30.445 us]
include_array/include_array_native_static
                        time:   [38.510 ns 38.640 ns 38.786 ns]
include_array/include_array_lazy_static
                        time:   [39.713 ns 39.863 ns 40.019 ns]
```

When using the **release** profile, the performance of `lazy_static_include_*` is very close to `include_*`. That means you don't need to worry about the overhead, but just enjoy the faster compilation time.

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