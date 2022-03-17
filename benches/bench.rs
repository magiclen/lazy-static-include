#[macro_use]
extern crate lazy_static_include;

use std::fs::File;
use std::io::Read;
use std::str::from_utf8_unchecked;

use bencher::{benchmark_group, benchmark_main, Bencher};
use slash_formatter::concat_with_file_separator;

macro_rules! benchmark_text_path {
    () => {
        concat_with_file_separator!(env!("CARGO_MANIFEST_DIR"), "data", "benchmark.txt")
    };
}

fn include_str_no_static(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut f = File::open(benchmark_text_path!()).unwrap();

        let mut v = Vec::new();

        f.read_to_end(&mut v).unwrap();

        let s = String::from_utf8(v).unwrap();

        s.contains("figarofigaro")
    });
}

fn include_str_native_static(bencher: &mut Bencher) {
    let text = include_str!(benchmark_text_path!());

    bencher.iter(|| text.contains("figarofigaro"));
}

fn include_str_lazy_static(bencher: &mut Bencher) {
    lazy_static_include_str! {
        pub TEXT => "data/benchmark.txt"
    }

    bencher.iter(|| TEXT.contains("figarofigaro"));
}

fn include_bytes_no_static(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut f = File::open(benchmark_text_path!()).unwrap();

        let mut v = Vec::new();

        f.read_to_end(&mut v).unwrap();

        let text = unsafe { from_utf8_unchecked(&v) };

        text.contains("figarofigaro")
    });
}

fn include_bytes_native_static(bencher: &mut Bencher) {
    let data = include_bytes!(benchmark_text_path!());

    bencher.iter(|| {
        let text = unsafe { from_utf8_unchecked(data) };

        text.contains("figarofigaro")
    });
}

fn include_bytes_lazy_static(bencher: &mut Bencher) {
    lazy_static_include_bytes! {
        DATA => "data/benchmark.txt"
    }

    bencher.iter(|| {
        let text = unsafe { from_utf8_unchecked(&DATA) };

        text.contains("figarofigaro")
    });
}

fn include_array_no_static(bencher: &mut Bencher) {
    let path = concat!(benchmark_text_path!());

    bencher.iter(|| {
        let mut f = File::open(&path).unwrap();

        let mut v = Vec::new();

        f.read_to_end(&mut v).unwrap();

        let array: Vec<&str> = serde_json::from_slice(&v).unwrap();

        array.binary_search(&"figarofigaro").is_ok()
    });
}

fn include_array_native_static(bencher: &mut Bencher) {
    let array = include!(benchmark_text_path!());

    bencher.iter(|| array.binary_search(&"figarofigaro").is_ok());
}

fn include_array_lazy_static(bencher: &mut Bencher) {
    lazy_static_include_array! {
        ARRAY: [&'static str; 622] => "data/benchmark.txt"
    }

    bencher.iter(|| ARRAY.binary_search(&"figarofigaro").is_ok());
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
