#[macro_use]
extern crate lazy_static_include;

#[macro_use]
extern crate lazy_static;

#[test]
fn test_include_str() {
    lazy_static_include_str!(TEST, "data/test.txt");

    lazy_static_include_str!(pub TEST2, "data/test-2.txt");

    assert_eq!("This is just a test text.", TEST);
    assert_eq!(TEST2, "Some text...");
}

#[test]
fn test_include_str_multiple() {
    lazy_static_include_str!(TEST, "data/test.txt", "data/test-2.txt");

    assert_eq!("This is just a test text.", TEST[0]);
    assert_eq!(TEST[1], "Some text...");

    lazy_static_include_str!(pub TEST2, "data/test-2.txt", "data/test.txt");

    assert_eq!("Some text...", TEST2[0]);
    assert_eq!(TEST2[1], "This is just a test text.");
}

#[test]
fn test_include_bytes() {
    lazy_static_include_bytes!(TEST, "data/test.txt");

    lazy_static_include_bytes!(pub TEST2, "data/test-2.txt");

    assert_eq!("This is just a test text.".as_bytes(), TEST);
    assert_eq!(TEST2, "Some text...".as_bytes());
}

#[test]
fn test_include_bytes_multiple() {
    lazy_static_include_bytes!(TEST, "data/test.txt", "data/test-2.txt");

    assert_eq!("This is just a test text.".as_bytes(), TEST[0]);
    assert_eq!(TEST[1], "Some text...".as_bytes());

    lazy_static_include_bytes!(pub TEST2, "data/test-2.txt", "data/test.txt");

    assert_eq!("Some text...".as_bytes(), TEST2[0]);
    assert_eq!(TEST2[1], "This is just a test text.".as_bytes());
}