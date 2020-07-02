/*!
# Lazy Static Include

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
assert_eq!("Some text...", TEST2);
```

```rust
#[macro_use] extern crate lazy_static_include;

lazy_static_include_bytes! {
    /// doc
    TEST => "data/test.txt",
    /// doc
    pub TEST2 => "data/test-2.txt",
}

assert_eq!("This is just a test text.".as_bytes(), TEST);
assert_eq!("Some text...".as_bytes(), TEST2);
```

You should notice that the value created from `lazy_static_include_bytes` and `lazy_static_include_str` macros isn't equal to `&'static [u8]` or `&'static str`. If you want to get an exact `&'static [u8]` or `&'static str` reference, you can **dereference** the value.

```rust
#[macro_use] extern crate lazy_static_include;

lazy_static_include_bytes! {
    /// doc
    TEST => "data/test.txt",
}

let data: &'static [u8] = *TEST;
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
test include_array_lazy_static   ... bench:          46 ns/iter (+/- 3)
test include_array_native_static ... bench:          48 ns/iter (+/- 3)
test include_array_no_static     ... bench:      22,414 ns/iter (+/- 297)
test include_bytes_lazy_static   ... bench:         844 ns/iter (+/- 3)
test include_bytes_native_static ... bench:         863 ns/iter (+/- 5)
test include_bytes_no_static     ... bench:       4,764 ns/iter (+/- 189)
test include_str_lazy_static     ... bench:         857 ns/iter (+/- 8)
test include_str_native_static   ... bench:         842 ns/iter (+/- 10)
test include_str_no_static       ... bench:       4,837 ns/iter (+/- 145)
```

When using the **release** profile, the performance of `lazy_static_include_*` is very close to `include_*`. That means you don't need to worry about the overhead, but just enjoy the faster compilation time.

You can run the benchmark program by executing,

```bash
cargo bench
```
*/

extern crate lazy_static;

extern crate slash_formatter;

#[doc(hidden)]
pub extern crate syn;

mod macro_include_array;
mod macro_include_bytes;
mod macro_include_str;

#[doc(hidden)]
pub use lazy_static::lazy_static;

#[doc(hidden)]
pub use slash_formatter::concat_with_file_separator;
