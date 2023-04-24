use lazy_static_include::*;

lazy_static_include_str! {
    pub STR => "data/test.txt",
    pub(crate) STR2 => "data/test.txt",
}

lazy_static_include_bytes! {
    pub BYTE => "data/test.txt",
    pub(crate) BYTE2 => "data/test.txt",
}

lazy_static_include_array! {
    pub ARRAY: [isize; 5] => "data/isize_array.txt",
    pub(crate) ARRAY2: [isize; 5] => "data/isize_array.txt",
}
