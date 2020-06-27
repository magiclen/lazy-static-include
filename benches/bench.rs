#[macro_use]
extern crate bencher;

#[macro_use]
extern crate lazy_static_include;

extern crate serde_json;

use std::fs::File;
use std::io::Read;

use bencher::Bencher;

fn include_str_no_static(bencher: &mut Bencher) {
    let path = concat!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "data/benchmark.txt"));

    bencher.iter(|| {
        let mut f = File::open(&path).unwrap();

        let mut v = Vec::new();

        f.read_to_end(&mut v).unwrap();

        let s = String::from_utf8(v).unwrap();

        assert!(s.contains("figarofigaro"));
    });
}

fn include_str_native_static(bencher: &mut Bencher) {
    let text = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "data/benchmark.txt"));

    bencher.iter(|| {
        assert!(text.contains("figarofigaro"));
    });
}

fn include_str_lazy_static(bencher: &mut Bencher) {
    lazy_static_include_str!(TEXT, "data/benchmark.txt");

    bencher.iter(|| {
        assert!((*TEXT).contains("figarofigaro"));
    });
}

fn include_bytes_no_static(bencher: &mut Bencher) {
    let path = concat!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "data/benchmark.txt"));

    bencher.iter(|| {
        let mut f = File::open(&path).unwrap();

        let mut v = Vec::new();

        f.read_to_end(&mut v).unwrap();

        String::from_utf8(v).unwrap();
    });
}

fn include_bytes_native_static(bencher: &mut Bencher) {
    let data = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "data/benchmark.txt"));

    bencher.iter(|| {
        String::from_utf8(data.to_vec()).unwrap();
    });
}

fn include_bytes_lazy_static(bencher: &mut Bencher) {
    lazy_static_include_bytes!(DATA, "data/benchmark.txt");

    bencher.iter(|| {
        String::from_utf8((*DATA).to_vec()).unwrap();
    });
}

fn include_array_no_static(bencher: &mut Bencher) {
    let path = concat!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "data/benchmark.txt"));

    bencher.iter(|| {
        let mut f = File::open(&path).unwrap();

        let mut v = Vec::new();

        f.read_to_end(&mut v).unwrap();

        let array: Vec<&str> = serde_json::from_slice(&v).unwrap();

        assert!(array.binary_search(&"figarofigaro").is_ok());
    });
}

fn include_array_native_static(bencher: &mut Bencher) {
    let array = include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "data/benchmark.txt"));

    bencher.iter(|| {
        assert!(array.binary_search(&"figarofigaro").is_ok());
    });
}

fn include_array_lazy_static(bencher: &mut Bencher) {
    lazy_static_include_array!(ARRAY: [&'static str; 622], "data/benchmark.txt");

    bencher.iter(|| {
        assert!((*ARRAY).binary_search(&"figarofigaro").is_ok());
    });
}

benchmark_group!(
    include_str,
    include_str_no_static,
    include_str_native_static,
    include_str_lazy_static
);
benchmark_group!(
    include_bytes,
    include_bytes_no_static,
    include_bytes_native_static,
    include_bytes_lazy_static
);
benchmark_group!(
    include_array,
    include_array_no_static,
    include_array_native_static,
    include_array_lazy_static
);

benchmark_main!(include_str, include_bytes, include_array);
