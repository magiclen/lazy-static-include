#![cfg(feature = "benchmark")]
#![feature(test)]

#[macro_use]
extern crate lazy_static_include;

#[macro_use]
extern crate lazy_static;

extern crate test;

use test::Bencher;

use std::fs::File;
use std::io::Read;

#[bench]
fn include_str_no_static(b: &mut Bencher) {
    let path = concat!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "data/benchmark.txt"));

    b.iter(|| {
        let mut f = File::open(&path).unwrap();

        let mut v = Vec::new();

        f.read_to_end(&mut v).unwrap();

        let s = String::from_utf8(v).unwrap();

        assert!(s.contains("figarofigaro"));
    });
}

#[bench]
fn include_str_native_static(b: &mut Bencher) {
    let text = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "data/benchmark.txt"));

    b.iter(|| {
        assert!(text.contains("figarofigaro"));
    });
}

#[bench]
fn include_str_lazy_static(b: &mut Bencher) {
    lazy_static_include_str!(TEXT, "data/benchmark.txt");

    b.iter(|| {
        assert!((*TEXT).contains("figarofigaro"));
    });
}

#[bench]
fn include_bytes_no_static(b: &mut Bencher) {
    let path = concat!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "data/benchmark.txt"));

    b.iter(|| {
        let mut f = File::open(&path).unwrap();

        let mut v = Vec::new();

        f.read_to_end(&mut v).unwrap();

        String::from_utf8(v).unwrap();
    });
}

#[bench]
fn include_bytes_native_static(b: &mut Bencher) {
    let data = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "data/benchmark.txt"));

    b.iter(|| {
        String::from_utf8(data.to_vec()).unwrap();
    });
}

#[bench]
fn include_bytes_lazy_static(b: &mut Bencher) {
    lazy_static_include_bytes!(DATA, "data/benchmark.txt");

    b.iter(|| {
        String::from_utf8((*DATA).to_vec()).unwrap();
    });
}

#[bench]
fn include_array_no_static(b: &mut Bencher) {
    let path = concat!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "data/benchmark.txt"));

    b.iter(|| {
        let mut f = File::open(&path).unwrap();

        let mut v = Vec::new();

        f.read_to_end(&mut v).unwrap();

        let s = String::from_utf8(v).unwrap();

        assert!(s.contains("figarofigaro"));
    });
}

#[bench]
fn include_array_native_static(b: &mut Bencher) {
    let text = include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "data/benchmark.txt"));

    b.iter(|| {
        assert!(text.binary_search(&"figarofigaro").is_ok());
    });
}

#[bench]
fn include_array_lazy_static(b: &mut Bencher) {
    lazy_static_include_array!(ARRAY: [&'static str; 622], "data/benchmark.txt");

    b.iter(|| {
        assert!((*ARRAY).binary_search(&"figarofigaro").is_ok());
    });
}