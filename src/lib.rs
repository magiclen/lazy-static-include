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

```rust,ignore
#[macro_use] extern crate lazy_static_include;
#[macro_use] extern crate lazy_static;

lazy_static_include_array!(TEST: [u64; 5], "data/u64_array.txt");
assert_eq!(123, TEST[0]);
assert_eq!(456, TEST[1]);
assert_eq!(789, TEST[2]);
assert_eq!(1000, TEST[3]);
assert_eq!(500000000000u64, TEST[4]);
```

```rust,ignore
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

```rust,ignore
#[macro_use] extern crate lazy_static_include;
#[macro_use] extern crate lazy_static;

lazy_static_include_array!(pub TEST: [&'static str; 3], "data/string_array.txt");

assert_eq!("Hi", TEST[0]);
assert_eq!("Hello", TEST[1]);
assert_eq!("哈囉", TEST[2]);
```

## Benchmark

Using static mechanisms makes your program faster. See my benchmark result below (Intel i7-6700HQ, ran on 2019/07/16):

```text
test include_array_lazy_static   ... bench:          43 ns/iter (+/- 3)
test include_array_native_static ... bench:          46 ns/iter (+/- 4)
test include_array_no_static     ... bench:      29,714 ns/iter (+/- 1,156)
test include_bytes_lazy_static   ... bench:         382 ns/iter (+/- 63)
test include_bytes_native_static ... bench:         380 ns/iter (+/- 30)
test include_bytes_no_static     ... bench:       9,076 ns/iter (+/- 1,224)
test include_str_lazy_static     ... bench:         932 ns/iter (+/- 103)
test include_str_native_static   ... bench:         937 ns/iter (+/- 25)
test include_str_no_static       ... bench:      10,135 ns/iter (+/- 1,634)
```

When using the **release** profile, the performance of `lazy_static_include_*` is very close to `include_*`. That means you don't need to worry about the overhead, but just enjoy the faster compilation time.

You can run the benchmark program by executing,

```bash
cargo bench
```
*/

#[doc(hidden)]
pub extern crate syn;

#[doc(hidden)]
pub extern crate starts_ends_with_caseless;

mod macro_include_array;
mod macro_include_bytes;
mod macro_include_counter;
mod macro_include_str;
