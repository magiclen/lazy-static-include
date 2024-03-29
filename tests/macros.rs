use assert_eq_float::assert_eq_float;
use lazy_static_include::*;

#[test]
fn include_str() {
    lazy_static_include_str! {
        TEST => "data/test.txt",
        TEST2 => "data/test-2.txt",
    }

    assert_eq!("This is just a test text.", TEST);
    assert_eq!("Some text...", TEST2);
}

#[test]
fn include_bytes() {
    lazy_static_include_bytes! {
        TEST => "data/test.txt",
        TEST2 => "data/test-2.txt",
    }

    assert_eq!("This is just a test text.".as_bytes(), TEST);
    assert_eq!("Some text...".as_bytes(), TEST2);
}

#[test]
fn include_array_isize() {
    lazy_static_include_array! {
        TEST: [isize; 5] => "data/isize_array.txt",
    }

    assert_eq!(123, TEST[0]);
    assert_eq!(-456, TEST[1]);
    assert_eq!(789, TEST[2]);
    assert_eq!(1000, TEST[3]);
    assert_eq!(5000, TEST[4]);
}

#[test]
fn include_array_i8() {
    lazy_static_include_array! {
        TEST: [i8; 5] => "data/i8_array.txt",
    }

    assert_eq!(12, TEST[0]);
    assert_eq!(-34, TEST[1]);
    assert_eq!(56, TEST[2]);
    assert_eq!(78, TEST[3]);
    assert_eq!(90, TEST[4]);
}

#[test]
fn include_array_i16() {
    lazy_static_include_array! {
        TEST: [i16; 5] => "data/i16_array.txt",
    }

    assert_eq!(123, TEST[0]);
    assert_eq!(-456, TEST[1]);
    assert_eq!(789, TEST[2]);
    assert_eq!(1000, TEST[3]);
    assert_eq!(5000, TEST[4]);
}

#[test]
fn include_array_i32() {
    lazy_static_include_array! {
        TEST: [i32; 5] => "data/i32_array.txt",
    }

    assert_eq!(123, TEST[0]);
    assert_eq!(-456, TEST[1]);
    assert_eq!(789, TEST[2]);
    assert_eq!(1000, TEST[3]);
    assert_eq!(5000, TEST[4]);
}

#[test]
fn include_array_i64() {
    lazy_static_include_array! {
        TEST: [i64; 5] => "data/i64_array.txt",
    }

    assert_eq!(123, TEST[0]);
    assert_eq!(-456, TEST[1]);
    assert_eq!(789, TEST[2]);
    assert_eq!(1000, TEST[3]);
    assert_eq!(500000000000i64, TEST[4]);
}

#[test]
fn include_array_i128() {
    lazy_static_include_array! {
        TEST: [i128; 5] => "data/i128_array.txt",
    }

    assert_eq!(123, TEST[0]);
    assert_eq!(-456, TEST[1]);
    assert_eq!(789, TEST[2]);
    assert_eq!(1000, TEST[3]);
    assert_eq!(500000000000000000000000i128, TEST[4]);
}

#[test]
fn include_array_usize() {
    lazy_static_include_array! {
        TEST: [usize; 5] => "data/usize_array.txt",
    }

    assert_eq!(123, TEST[0]);
    assert_eq!(456, TEST[1]);
    assert_eq!(789, TEST[2]);
    assert_eq!(1000, TEST[3]);
    assert_eq!(5000, TEST[4]);
}

#[test]
fn include_array_u8() {
    lazy_static_include_array! {
        TEST: [u8; 5] => "data/u8_array.txt",
    }

    assert_eq!(12, TEST[0]);
    assert_eq!(34, TEST[1]);
    assert_eq!(56, TEST[2]);
    assert_eq!(78, TEST[3]);
    assert_eq!(90, TEST[4]);
}

#[test]
fn include_array_u16() {
    lazy_static_include_array! {
        TEST: [u16; 5] => "data/u16_array.txt",
    }

    assert_eq!(123, TEST[0]);
    assert_eq!(456, TEST[1]);
    assert_eq!(789, TEST[2]);
    assert_eq!(1000, TEST[3]);
    assert_eq!(5000, TEST[4]);
}

#[test]
fn include_array_u32() {
    lazy_static_include_array! {
        TEST: [u32; 5] => "data/u32_array.txt",
    }

    assert_eq!(123, TEST[0]);
    assert_eq!(456, TEST[1]);
    assert_eq!(789, TEST[2]);
    assert_eq!(1000, TEST[3]);
    assert_eq!(5000, TEST[4]);
}

#[test]
fn include_array_u64() {
    lazy_static_include_array! {
        TEST: [u64; 5] => "data/u64_array.txt",
    }

    assert_eq!(123, TEST[0]);
    assert_eq!(456, TEST[1]);
    assert_eq!(789, TEST[2]);
    assert_eq!(1000, TEST[3]);
    assert_eq!(500000000000u64, TEST[4]);
}

#[test]
fn include_array_u128() {
    lazy_static_include_array! {
        TEST: [u128; 5] => "data/u128_array.txt",
    }

    assert_eq!(123, TEST[0]);
    assert_eq!(456, TEST[1]);
    assert_eq!(789, TEST[2]);
    assert_eq!(1000, TEST[3]);
    assert_eq!(500000000000000000000000u128, TEST[4]);
}

#[test]
fn include_array_f32() {
    lazy_static_include_array! {
        TEST: [f32; 5] => "data/f32_array.txt",
    }

    assert_eq_float!(123f32, TEST[0]);
    assert_eq_float!(-456f32, TEST[1]);
    assert_eq_float!(789.5f32, TEST[2]);
    assert_eq_float!(1000.123f32, TEST[3]);
    assert_eq_float!(5000f32, TEST[4]);
}

#[test]
fn include_array_f64() {
    lazy_static_include_array! {
        TEST: [f64; 5] => "data/f64_array.txt",
    }

    assert_eq_float!(123f64, TEST[0]);
    assert_eq_float!(-456f64, TEST[1]);
    assert_eq_float!(789.5f64, TEST[2]);
    assert_eq_float!(1000.123f64, TEST[3]);
    assert_eq_float!(5000.456f64, TEST[4]);
}

#[test]
fn include_array_char() {
    lazy_static_include_array! {
        TEST: [char; 3] => "data/char_array.txt",
    }

    assert_eq!('a', TEST[0]);
    assert_eq!('b', TEST[1]);
    assert_eq!('c', TEST[2]);
}

#[test]
fn include_array_bool() {
    lazy_static_include_array! {
        TEST: [bool; 3] => "data/bool_array.txt",
    }

    assert!(!TEST[0]);
    assert!(TEST[1]);
    assert!(!TEST[2]);
}

#[test]
fn include_array_string() {
    lazy_static_include_array! {
        TEST: [&'static str; 3] => "data/string_array.txt",
    }

    assert_eq!("Hi", TEST[0]);
    assert_eq!("Hello", TEST[1]);
    assert_eq!("哈囉", TEST[2]);
}
