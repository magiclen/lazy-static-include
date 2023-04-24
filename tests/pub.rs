mod test_mod;

#[test]
fn include_str() {
    assert_eq!("This is just a test text.", test_mod::STR);
    assert_eq!("This is just a test text.", test_mod::STR2);
}

#[test]
fn include_bytes() {
    assert_eq!("This is just a test text.".as_bytes(), test_mod::BYTE);
    assert_eq!("This is just a test text.".as_bytes(), test_mod::BYTE2);
}

#[test]
fn include_array() {
    assert_eq!(123, test_mod::ARRAY[0]);
    assert_eq!(123, test_mod::ARRAY2[0]);
}
