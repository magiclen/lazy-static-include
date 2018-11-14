#[macro_use]
extern crate lazy_static_include;

#[macro_use]
extern crate lazy_static;

#[test]
fn include_str() {
    lazy_static_include_str!(TEST, "data/test.txt");

    lazy_static_include_str!(pub TEST2, "data/test-2.txt");

    assert_eq!("This is just a test text.", TEST);
    assert_eq!(TEST2, "Some text...");
}

#[test]
fn include_str_vec() {
    lazy_static_include_str_vec!(TEST, "data/test.txt");

    assert_eq!("This is just a test text.", TEST[0]);

    lazy_static_include_str_vec!(pub TEST2, "data/test-2.txt");

    assert_eq!(TEST2[0], "Some text...");
}

#[test]
fn include_str_multiple() {
    lazy_static_include_str!(TEST, "data/test.txt", "data/test-2.txt");

    assert_eq!("This is just a test text.", TEST[0]);
    assert_eq!(TEST[1], "Some text...");

    lazy_static_include_str!(pub TEST2, "data/test-2.txt", "data/test.txt");

    assert_eq!("Some text...", TEST2[0]);
    assert_eq!(TEST2[1], "This is just a test text.");
}

#[test]
fn include_bytes() {
    lazy_static_include_bytes!(TEST, "data/test.txt");

    lazy_static_include_bytes!(pub TEST2, "data/test-2.txt");

    assert_eq!("This is just a test text.".as_bytes(), TEST);
    assert_eq!(TEST2, "Some text...".as_bytes());
}

#[test]
fn include_bytes_vec() {
    lazy_static_include_bytes_vec!(TEST, "data/test.txt");

    assert_eq!("This is just a test text.".as_bytes(), TEST[0]);

    lazy_static_include_bytes_vec!(pub TEST2, "data/test-2.txt");

    assert_eq!(TEST2[0], "Some text...".as_bytes());
}

#[test]
fn include_bytes_multiple() {
    lazy_static_include_bytes!(TEST, "data/test.txt", "data/test-2.txt");

    assert_eq!("This is just a test text.".as_bytes(), TEST[0]);
    assert_eq!(TEST[1], "Some text...".as_bytes());

    lazy_static_include_bytes!(pub TEST2, "data/test-2.txt", "data/test.txt");

    assert_eq!("Some text...".as_bytes(), TEST2[0]);
    assert_eq!(TEST2[1], "This is just a test text.".as_bytes());
}

#[test]
fn include_array_i8() {
    lazy_static_include_array!(TEST: [i8; 5], "data/i8_array.txt");

    assert_eq!(12, TEST[0]);
    assert_eq!(-34, TEST[1]);
    assert_eq!(56, TEST[2]);
    assert_eq!(78, TEST[3]);
    assert_eq!(90, TEST[4]);
}

#[test]
fn include_array_i16() {
    lazy_static_include_array!(pub TEST: [i16; 5], "data/i16_array.txt");

    assert_eq!(123, TEST[0]);
    assert_eq!(-456, TEST[1]);
    assert_eq!(789, TEST[2]);
    assert_eq!(1000, TEST[3]);
    assert_eq!(5000, TEST[4]);
}

#[test]
fn include_array_i32() {
    lazy_static_include_array!(TEST: [i32; 5], "data/i32_array.txt");

    assert_eq!(123, TEST[0]);
    assert_eq!(-456, TEST[1]);
    assert_eq!(789, TEST[2]);
    assert_eq!(1000, TEST[3]);
    assert_eq!(5000, TEST[4]);
}

#[test]
fn include_array_i64() {
    lazy_static_include_array!(pub TEST: [i64; 5], "data/i64_array.txt");

    assert_eq!(123, TEST[0]);
    assert_eq!(-456, TEST[1]);
    assert_eq!(789, TEST[2]);
    assert_eq!(1000, TEST[3]);
    assert_eq!(500000000000i64, TEST[4]);
}

#[test]
fn include_array_i128() {
    lazy_static_include_array!(TEST: [i128; 5], "data/i128_array.txt");

    assert_eq!(123, TEST[0]);
    assert_eq!(-456, TEST[1]);
    assert_eq!(789, TEST[2]);
    assert_eq!(1000, TEST[3]);
    assert_eq!(500000000000000000000000i128, TEST[4]);
}

#[test]
fn include_array_f32() {
    lazy_static_include_array!(pub TEST: [f32; 5], "data/f32_array.txt");

    assert_eq!(123f32, TEST[0]);
    assert_eq!(-456f32, TEST[1]);
    assert_eq!(789.5f32, TEST[2]);
    assert_eq!(1000.123f32, TEST[3]);
    assert_eq!(5000f32, TEST[4]);
}

#[test]
fn include_array_f64() {
    lazy_static_include_array!(TEST: [f64; 5], "data/f64_array.txt");

    assert_eq!(123f64, TEST[0]);
    assert_eq!(-456f64, TEST[1]);
    assert_eq!(789.5f64, TEST[2]);
    assert_eq!(1000.123f64, TEST[3]);
    assert_eq!(5000.456f64, TEST[4]);
}

#[test]
fn include_array_u8() {
    lazy_static_include_array!(pub TEST: [u8; 5], "data/u8_array.txt");

    assert_eq!(12, TEST[0]);
    assert_eq!(34, TEST[1]);
    assert_eq!(56, TEST[2]);
    assert_eq!(78, TEST[3]);
    assert_eq!(90, TEST[4]);
}

#[test]
fn include_array_u16() {
    lazy_static_include_array!(TEST: [u16; 5], "data/u16_array.txt");

    assert_eq!(123, TEST[0]);
    assert_eq!(456, TEST[1]);
    assert_eq!(789, TEST[2]);
    assert_eq!(1000, TEST[3]);
    assert_eq!(5000, TEST[4]);
}

#[test]
fn include_array_u32() {
    lazy_static_include_array!(pub TEST: [u32; 5], "data/u32_array.txt");

    assert_eq!(123, TEST[0]);
    assert_eq!(456, TEST[1]);
    assert_eq!(789, TEST[2]);
    assert_eq!(1000, TEST[3]);
    assert_eq!(5000, TEST[4]);
}

#[test]
fn include_array_u64() {
    lazy_static_include_array!(TEST: [u64; 5], "data/u64_array.txt");

    assert_eq!(123, TEST[0]);
    assert_eq!(456, TEST[1]);
    assert_eq!(789, TEST[2]);
    assert_eq!(1000, TEST[3]);
    assert_eq!(500000000000u64, TEST[4]);
}

#[test]
fn include_array_u128() {
    lazy_static_include_array!(pub TEST: [u128; 5], "data/u128_array.txt");

    assert_eq!(123, TEST[0]);
    assert_eq!(456, TEST[1]);
    assert_eq!(789, TEST[2]);
    assert_eq!(1000, TEST[3]);
    assert_eq!(500000000000000000000000u128, TEST[4]);
}

#[test]
fn include_array_i32_multiple() {
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
}

#[test]
fn include_array_string() {
    lazy_static_include_array!(pub TEST: [&'static str; 3], "data/string_array.txt");

    assert_eq!("Hi", TEST[0]);
    assert_eq!("Hello", TEST[1]);
    assert_eq!("哈囉", TEST[2]);
}

#[test]
fn include_array_string_vec() {
    lazy_static_include_array_vec!(TEST: [&'static str; 3], "data/string_array.txt");

    assert_eq!("Hi", TEST[0][0]);
    assert_eq!("Hello", TEST[0][1]);
    assert_eq!("哈囉", TEST[0][2]);
}

#[test]
fn include_array_string_multiple() {
    lazy_static_include_array!(TEST: [&'static str; 3], "data/string_array.txt", "data/string_array-2.txt");

    assert_eq!("Hi", TEST[0][0]);
    assert_eq!("Hello", TEST[0][1]);
    assert_eq!("哈囉", TEST[0][2]);

    assert_eq!("Hello world!", TEST[1][0]);
    assert_eq!("Rust", TEST[1][1]);
    assert_eq!("你好", TEST[1][2]);
}

#[test]
fn include_array_char() {
    lazy_static_include_array!(pub TEST: [char; 3], "data/char_array.txt");

    assert_eq!('a', TEST[0]);
    assert_eq!('b', TEST[1]);
    assert_eq!('c', TEST[2]);
}

#[test]
fn include_array_bool() {
    lazy_static_include_array!(pub TEST: [bool; 3], "data/bool_array.txt");

    assert_eq!(false, TEST[0]);
    assert_eq!(true, TEST[1]);
    assert_eq!(false, TEST[2]);
}