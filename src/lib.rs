//! # Lazy Static Include
//!
//! This crate provides `lazy_static_include_bytes` and `lazy_static_include_str` macros to replace `include_bytes` and `include_str` macros.
//!
//! Why should we do that?
//! Because the original `include_bytes` and `include_str` macros bring extra data from files into the compiled executable binary file, the time for compiling surges.
//!
//! High compilation time is detrimental to software development. `lazy_static_include_bytes` and `lazy_static_include_str` macros can help you **lazy load** data from files
//! when you are not using the **release** profile. In other words, if you are using `include_bytes` and `include_str` macros, and you think your compilation time is too high to wait,
//! you can choose to use `lazy_static_include_bytes` and `lazy_static_include_str` macros.
//!
//! `lazy_static_include_bytes` and `lazy_static_include_str` macros include data from files into the compiled executable binary file **only** when you are using the **release** profile.
//! Be careful when you distribute your program.
//!
//! The paths used for `lazy_static_include_bytes` and `lazy_static_include_str` are relative to **CARGO_MANIFEST_DIR**.
//!
//! ## Examples
//!
//! ```
//! #[macro_use] extern crate lazy_static_include;
//! #[macro_use] extern crate lazy_static;
//!
//! lazy_static_include_str!(TEST, "data/test.txt");
//! lazy_static_include_str!(pub TEST2, "data/test-2.txt");
//!
//! assert_eq!("This is just a test text.", TEST);
//! assert_eq!(TEST2, "Some text...");
//! ```
//!
//! ```
//! #[macro_use] extern crate lazy_static_include;
//! #[macro_use] extern crate lazy_static;
//!
//! lazy_static_include_bytes!(TEST, "data/test.txt", "data/test-2.txt");
//!
//! assert_eq!("This is just a test text.".as_bytes(), TEST[0]);
//! assert_eq!(TEST[1], "Some text...".as_bytes());
//! ```
//!
//! You should notice that the struct created from `lazy_static_include_bytes` and `lazy_static_include_str` macros isn't equal to `&'static [u8]` or `&'static str`.
//! If you want to get an exact `&'static [u8]` or `&'static str` reference, you need to **dereference the struct**.
//!
//! ```
//! #[macro_use] extern crate lazy_static_include;
//! #[macro_use] extern crate lazy_static;
//!
//! lazy_static_include_bytes!(TEST, "data/test.txt");
//!
//! let data: &'static [u8] = *TEST;
//! ```
//!
//! If you include str and bytes from multiple files, after dereferencing the struct, you will get a `Vec<&'static [u8]>` or a `Vec<&'static str>`.
//! In order to not move out of borrowed content, use **&*** to get the reference of that `Vec`.
//!
//! ```
//! #[macro_use] extern crate lazy_static_include;
//! #[macro_use] extern crate lazy_static;
//!
//! lazy_static_include_str!(TEST, "data/test.txt", "data/test-2.txt");
//!
//! let v: &Vec<&'static str> = &*TEST;
//! ```
//!
//! ## Include Array
//!
//! There is a special macro `lazy_static_include_array` which can include arrays from files.
//! The array is fixed sized and can be one of these following types: `bool`, `char`, `u8`, `u16`, `u32`, `u64`, `u128`, `i8`, `i16`, `i32`, `i64`, `i128`, `f32`, `f64`, `&'static str`.
//!
//! Also, the `lazy_static_include_array` macro includes data from files into the compiled executable binary file **only** when you are using the **release** profile.
//! Be careful when you distribute your program.
//!
//! ```
//! #[macro_use] extern crate lazy_static_include;
//! #[macro_use] extern crate lazy_static;
//!
//! lazy_static_include_array!(TEST: [u64; 5], "data/u64_array.txt");
//! assert_eq!(123, TEST[0]);
//! assert_eq!(456, TEST[1]);
//! assert_eq!(789, TEST[2]);
//! assert_eq!(1000, TEST[3]);
//! assert_eq!(500000000000u64, TEST[4]);
//! ```
//!
//! ```
//! #[macro_use] extern crate lazy_static_include;
//! #[macro_use] extern crate lazy_static;
//!
//! lazy_static_include_array!(TEST: [i32; 5], "data/i32_array.txt", "data/i32_array-2.txt");
//! assert_eq!(123, TEST[0][0]);
//! assert_eq!(-456, TEST[0][1]);
//! assert_eq!(789, TEST[0][2]);
//! assert_eq!(1000, TEST[0][3]);
//! assert_eq!(5000, TEST[0][4]);
//!
//! assert_eq!(-1, TEST[1][0]);
//! assert_eq!(-2, TEST[1][1]);
//! assert_eq!(-3, TEST[1][2]);
//! assert_eq!(-4, TEST[1][3]);
//! assert_eq!(-5, TEST[1][4]);
//! ```
//! ```
//! #[macro_use] extern crate lazy_static_include;
//! #[macro_use] extern crate lazy_static;
//!
//! lazy_static_include_array!(pub TEST: [&'static str; 3], "data/string_array.txt");
//!
//! assert_eq!("Hi", TEST[0]);
//! assert_eq!("Hello", TEST[1]);
//! assert_eq!("哈囉", TEST[2]);
//! ```

#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_counter {
    () => (0usize);
    ( $x:expr $(, $xs:expr)* $(,)* ) => (1usize + lazy_static_include_counter!($($xs, )*));
}

// TODO -----include_str START-----

#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_str_impl {
    ( $name:ident ) => {
        impl ::std::cmp::PartialEq<str> for $name {
            fn eq(&self, other: &str) -> bool{
                (*$name).eq(other)
            }

            fn ne(&self, other: &str) -> bool {
                (*$name).ne(other)
            }
        }

        impl<'a> ::std::cmp::PartialEq<&'a str> for $name {
            fn eq(&self, other: &&'a str) -> bool{
                (&*$name).eq(other)
            }

            fn ne(&self, other: &&'a str) -> bool {
                (&*$name).ne(other)
            }
        }

        impl ::std::cmp::PartialEq for $name {
            fn eq(&self, other: &$name) -> bool{
                true
            }

            fn ne(&self, other: &$name) -> bool {
                false
            }
        }

        impl<'a> ::std::cmp::PartialEq<$name> for &'a str {
            fn eq(&self, other: &$name) -> bool{
                self.eq(&*$name)
            }

            fn ne(&self, other: &$name) -> bool {
                self.ne(&*$name)
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Debug::fmt(*$name, f)
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(*$name, f)
            }
        }

        impl<T: ?Sized> AsRef<T> for $name where str: ::std::convert::AsRef<T>{
            fn as_ref(&self) -> &T{
                (*$name).as_ref()
            }
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_str_multiple_impl {
    ( $name:ident ) => {
        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Debug::fmt(&*$name, f)
            }
        }
    }
}

#[cfg(not(debug_assertions))]
#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_str_inner {
    ( $name:ident, $path:expr ) => {
        {
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path))
        }
    };
    ( $name:ident, $path:expr, $($paths:expr), + ) => {
        {
            let mut v: Vec<&'static str> = Vec::with_capacity(lazy_static_include_counter!($path $(, $paths)+));

            v.push(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path)));

            $(
                v.push(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $paths)));
            )+

            v
        }
    };
}

#[cfg(debug_assertions)]
#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_str_inner {
    ( $name:ident, $path:expr ) => {
        {
            use ::std::fs::File;
            use ::std::io::Read;
            use ::std::mem;

            let v = {
                let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", $path);

                let mut f = File::open(&path).unwrap();

                let mut v: Vec<u8> = Vec::new();

                f.read_to_end(&mut v).unwrap();

                v
            };

            let s = String::from_utf8(v).unwrap();

            unsafe {
                let ret = mem::transmute(s.as_str());
                mem::forget(s);
                ret
            }
        }
    };
    ( $name:ident, $path:expr, $($paths:expr), + ) => {
        {
            use ::std::fs::File;
            use ::std::io::Read;
            use ::std::mem;

            let mut v: Vec<&'static str> = Vec::with_capacity(lazy_static_include_counter!($path $(, $paths)+));

            let vv = {
                let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", $path);

                let mut f = File::open(&path).unwrap();

                let mut v: Vec<u8> = Vec::new();

                f.read_to_end(&mut v).unwrap();

                v
            };

            let s = String::from_utf8(vv).unwrap();

            v.push(
                unsafe {
                    let ret = mem::transmute(s.as_str());
                    mem::forget(s);
                    ret
                }
            );

            $(
                let vv = {
                    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", $paths);

                    let mut f = File::open(&path).unwrap();

                    let mut v: Vec<u8> = Vec::new();

                    f.read_to_end(&mut v).unwrap();

                    v
                };

                let s = String::from_utf8(vv).unwrap();

                v.push(
                    unsafe {
                        let ret = mem::transmute(s.as_str());
                        mem::forget(s);
                        ret
                    }
                );
            )+

            v
        }
    };
}

#[macro_export]
macro_rules! lazy_static_include_str {
    ( $name:ident, $path:expr ) => {
        lazy_static! {
            static ref $name: &'static str = lazy_static_include_str_inner!($name, $path);
        }

        lazy_static_include_str_impl!($name);
    };
    ( $name:ident, $path:expr, $($paths:expr), + ) => {
        lazy_static! {
            static ref $name: Vec<&'static str> = lazy_static_include_str_inner!($name, $path $(, $paths)+);
        }

        lazy_static_include_str_multiple_impl!($name);
    };
    ( pub $name:ident, $path:expr ) => {
        lazy_static! {
            pub static ref $name: &'static str = lazy_static_include_str_inner!($name, $path);
        }

        lazy_static_include_str_impl!($name);
    };
    ( pub $name:ident, $path:expr, $($paths:expr), + ) => {
        lazy_static! {
            static ref $name: Vec<&'static str> = lazy_static_include_str_inner!($name, $path $(, $paths)+);
        }

        lazy_static_include_str_multiple_impl!($name);
    };
}

// TODO -----include_str END-----

// TODO -----include_bytes START-----

#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_bytes_impl {
    ( $name:ident ) => {
        impl<'a> ::std::cmp::PartialEq<&'a [u8]> for $name {
            fn eq(&self, other: &&'a [u8]) -> bool{
                (&*$name).eq(other)
            }

            fn ne(&self, other: &&'a [u8]) -> bool {
                (&*$name).ne(other)
            }
        }

        impl ::std::cmp::PartialEq for $name {
            fn eq(&self, other: &$name) -> bool{
                true
            }

            fn ne(&self, other: &$name) -> bool {
                false
            }
        }

        impl<'a> ::std::cmp::PartialEq<$name> for &'a [u8] {
            fn eq(&self, other: &$name) -> bool{
                self.eq(&*$name)
            }

            fn ne(&self, other: &$name) -> bool {
                self.ne(&*$name)
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Debug::fmt(*$name, f)
            }
        }

        impl AsRef<[u8]> for $name {
            fn as_ref(&self) -> &[u8] {
                (*$name).as_ref()
            }
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_bytes_multiple_impl {
    ( $name:ident ) => {
        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Debug::fmt(&*$name, f)
            }
        }
    }
}

#[cfg(not(debug_assertions))]
#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_bytes_inner {
    ( $name:ident, $path:expr ) => {
        {
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path))
        }
    };
    ( $name:ident, $path:expr, $($paths:expr), + ) => {
        {
            let mut v: Vec<&'static [u8]> = Vec::with_capacity(lazy_static_include_counter!($path $(, $paths)+));

            v.push(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path)));

            $(
                v.push(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $paths)));
            )+

            v
        }
    };
}

#[cfg(debug_assertions)]
#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_bytes_inner {
    ( $name:ident, $path:expr ) => {
        {
            use ::std::fs::File;
            use ::std::io::Read;
            use ::std::mem;

            let v = {
                let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", $path);

                let mut f = File::open(&path).unwrap();

                let mut v: Vec<u8> = Vec::new();

                f.read_to_end(&mut v).unwrap();

                v
            };

            unsafe {
                let ret = mem::transmute(v.as_ref() as &[u8]);
                mem::forget(v);
                ret
            }
        }
    };
    ( $name:ident, $path:expr, $($paths:expr), + ) => {
        {
            use ::std::fs::File;
            use ::std::io::Read;
            use ::std::mem;

            let mut v: Vec<&'static [u8]> = Vec::with_capacity(lazy_static_include_counter!($path $(, $paths)+));

            let vv = {
                let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", $path);

                let mut f = File::open(&path).unwrap();

                let mut v: Vec<u8> = Vec::new();

                f.read_to_end(&mut v).unwrap();

                v
            };

            v.push(
                unsafe {
                    let ret = mem::transmute(vv.as_ref() as &[u8]);
                    mem::forget(vv);
                    ret
                }
            );

            $(
                let vv = {
                    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", $paths);

                    let mut f = File::open(&path).unwrap();

                    let mut v: Vec<u8> = Vec::new();

                    f.read_to_end(&mut v).unwrap();

                    v
                };

                v.push(
                    unsafe {
                        let ret = mem::transmute(vv.as_ref() as &[u8]);
                        mem::forget(vv);
                        ret
                    }
                );
            )+

            v
        }
    };
}

#[macro_export]
macro_rules! lazy_static_include_bytes {
    ( $name:ident, $path:expr ) => {
        lazy_static! {
            static ref $name: &'static [u8] = lazy_static_include_bytes_inner!($name, $path);
        }

        lazy_static_include_bytes_impl!($name);
    };
    ( $name:ident, $path:expr, $($paths:expr), + ) => {
        lazy_static! {
            static ref $name: Vec<&'static [u8]> = lazy_static_include_bytes_inner!($name, $path $(, $paths)+);
        }

        lazy_static_include_bytes_multiple_impl!($name);
    };
    ( pub $name:ident, $path:expr ) => {
        lazy_static! {
            pub static ref $name: &'static [u8] = lazy_static_include_bytes_inner!($name, $path);
        }

        lazy_static_include_bytes_impl!($name);
    };
    ( pub $name:ident, $path:expr, $($paths:expr), + ) => {
        lazy_static! {
            static ref $name: Vec<&'static [u8]> = lazy_static_include_bytes_inner!($name, $path $(, $paths)+);
        }

        lazy_static_include_bytes_multiple_impl!($name);
    };
}

// TODO -----include_bytes END-----

// TODO -----include_array START-----

#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_array_impl {
    ( $name:ident ) => {
        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Debug::fmt(&*$name, f)
            }
        }
    }
}

#[cfg(not(debug_assertions))]
#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_array_inner {
    ( $name:ident: [&'static str; $s:expr], $path:expr ) => {
        {
            include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path))
        }
    };
    ( $name:ident: [&'static str; $s:expr], $path:expr, $($paths:expr), + ) => {
        {
            let mut v: Vec<[&'static str; $s]> = Vec::with_capacity(lazy_static_include_counter!($path $(, $paths)+));

            v.push(include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path)));

            $(
                v.push(include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $paths)));
            )+

            v
        }
    };
    ( $name:ident: [$t:ident; $s:expr], $path:expr ) => {
        {
            include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path))
        }
    };
    ( $name:ident: [$t:ident; $s:expr], $path:expr, $($paths:expr), + ) => {
        {
            let mut v: Vec<[$t; $s]> = Vec::with_capacity(lazy_static_include_counter!($path $(, $paths)+));

            v.push(include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path)));

            $(
                v.push(include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $paths)));
            )+

            v
        }
    };
}

#[cfg(debug_assertions)]
#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_array_inner_b {
    ( $name:ident: [$t:ident; $s:expr], $path:expr ) => {
        {
            use ::std::fs::File;
            use ::std::io::Read;
            use ::std::mem;

            let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", $path);

            let v = {
                let mut f = File::open(&path).unwrap();

                let mut v: Vec<u8> = Vec::new();

                f.read_to_end(&mut v).unwrap();

                v
            };

            let s = String::from_utf8(v).unwrap();

            let s = s.trim();

            let len = s.len();

            if len < 2 || &s[0..1] != "[" || &s[len - 1..len] != "]" {
                panic!("incorrect array, file: {}", path);
            }

            let mut result = [false; $s];
            let mut p = 0usize;

            let mut concating = false;
            let mut tmp = String::new();

            let len = s.chars().count();

            for c in s.chars().into_iter().skip(1).take(len - 2) {
                 if c == ' ' || c == '\n'  || c == '\t'{
                    if concating {
                        if c == '\n' {
                            panic!("incorrect array, file: {}", path);
                        } else {
                            tmp.push(c);
                        }
                    }else {
                        continue;
                    }
                } else if c == ',' {
                    if concating {
                        panic!("incorrect array, file: {}", path);
                    }
                } else if c == 't' || c == 'f' {
                    if concating {
                        panic!("incorrect array, file: {}", path);
                    } else {
                        concating = true;

                        tmp.push(c);
                    }
                } else if c == 'e' {
                    if concating {
                        if "tru".eq(tmp.as_str()) {
                            result[p] = true;
                        } else if "fals".eq(tmp.as_str()) {
                            result[p] = false;
                        } else {
                            panic!("incorrect array, file: {}", path);
                        }

                        p += 1;

                        tmp.clear();

                        concating = false;
                    } else {
                        panic!("incorrect array, file: {}", path);
                    }
                } else {
                    if concating {
                        tmp.push(c);
                    } else {
                        panic!("incorrect array, file: {}", path);
                    }
                }
            }

            if concating {
                panic!("incorrect array, file: {}", path);
            }

            if p != $s {
                panic!("incorrect array, file: {}", path);
            }

            result
        }
    }
}

#[cfg(debug_assertions)]
#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_array_inner_c {
    ( $name:ident: [$t:ident; $s:expr], $path:expr ) => {
        {
            use ::std::fs::File;
            use ::std::io::Read;
            use ::std::mem;

            let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", $path);

            let v = {
                let mut f = File::open(&path).unwrap();

                let mut v: Vec<u8> = Vec::new();

                f.read_to_end(&mut v).unwrap();

                v
            };

            let s = String::from_utf8(v).unwrap();

            let s = s.trim();

            let len = s.len();

            if len < 2 || &s[0..1] != "[" || &s[len - 1..len] != "]" {
                panic!("incorrect array, file: {}", path);
            }

            let mut result = ['\0'; $s];
            let mut p = 0usize;

            let mut concating = false;
            let mut cannot_concating = false;
            let mut tmp = '\0';

            let len = s.chars().count();

            for c in s.chars().into_iter().skip(1).take(len - 2) {
                 if c == ' ' || c == '\n'  || c == '\t'{
                    if concating {
                        if c == '\n' {
                            panic!("incorrect array, file: {}", path);
                        } else {
                            tmp = c;
                        }
                    }else {
                        continue;
                    }
                } else if c == ',' {
                    if concating {
                        if cannot_concating {
                            panic!("incorrect array, file: {}", path);
                        }
                        tmp = c;
                        cannot_concating = true;
                    } else {
                        cannot_concating = false;
                    }
                } else if c == '\'' {
                    if concating {
                        result[p] = tmp;
                        p += 1;

                        concating = false;
                    } else {
                        if cannot_concating {
                            panic!("incorrect array, file: {}", path);
                        }
                        concating = true;
                    }
                } else {
                    if concating {
                        if cannot_concating {
                            panic!("incorrect array, file: {}", path);
                        }
                        tmp = c;
                        cannot_concating = true;
                    } else {
                        panic!("incorrect array, file: {}", path);
                    }
                }
            }

            if concating {
                panic!("incorrect array, file: {}", path);
            }

            if p != $s {
                panic!("incorrect array, file: {}", path);
            }

            result
        }
    }
}

#[cfg(debug_assertions)]
#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_array_inner_s {
    ( $name:ident: [&'static str; $s:expr], $path:expr ) => {
        {
            use ::std::fs::File;
            use ::std::io::Read;
            use ::std::mem;

            let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", $path);

            let v = {
                let mut f = File::open(&path).unwrap();

                let mut v: Vec<u8> = Vec::new();

                f.read_to_end(&mut v).unwrap();

                v
            };

            let s = String::from_utf8(v).unwrap();

            let s = s.trim();

            let len = s.len();

            if len < 2 || &s[0..1] != "[" || &s[len - 1..len] != "]" {
                panic!("incorrect array, file: {}", path);
            }

            let mut result = Vec::with_capacity($s);

            let mut concating = false;
            let mut cannot_concating = false;
            let mut tmp = String::new();

            let len = s.chars().count();

            for c in s.chars().into_iter().skip(1).take(len - 2) {
                 if c == ' ' || c == '\n'  || c == '\t'{
                    if concating {
                        if c == '\n' {
                            panic!("incorrect array, file: {}", path);
                        } else {
                            tmp.push(c);
                        }
                    }else {
                        continue;
                    }
                } else if c == ',' {
                    if concating {
                        result.push(tmp.clone());
                    } else {
                        cannot_concating = false;
                    }
                } else if c == '"' {
                    if concating {
                        result.push(tmp.clone());

                        concating = false;

                        tmp.clear();

                        cannot_concating = true;
                    } else {
                        if cannot_concating {
                            panic!("incorrect array, file: {}", path);
                        }
                        concating = true;
                    }
                } else {
                    if concating {
                        tmp.push(c);
                    } else {
                        panic!("incorrect array, file: {}", path);
                    }
                }
            }

            if concating {
                panic!("incorrect array, file: {}", path);
            }

            if result.len() != $s {
                panic!("incorrect array, file: {}", path);
            }

            let mut result_str = [""; $s];

            for (i, s) in result.iter().enumerate() {
                result_str[i] = unsafe {
                    let ret = mem::transmute(s.as_str());
                    ret
                };
            }

            unsafe {
                mem::forget(result);
            };

            result_str
        }
    }
}

#[cfg(debug_assertions)]
#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_array_inner_u {
    ( $name:ident: [$t:ident; $s:expr], $path:expr ) => {
        {
            use ::std::fs::File;
            use ::std::io::Read;

            let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", $path);

            let v = {
                let mut f = File::open(&path).unwrap();

                let mut v: Vec<u8> = Vec::new();

                f.read_to_end(&mut v).unwrap();

                v
            };

            let s = String::from_utf8(v).unwrap();

            let s = s.trim();

            let len = s.len();

            if len < 2 || &s[0..1] != "[" || &s[len - 1..len] != "]" {
                panic!("incorrect array, file: {}", path);
            }

            let mut result = [0 as $t; $s];

            let mut p = 0usize;

            let mut concating = false;
            let mut concating_ignore = false;
            let mut concating_ignore_s = String::new();
            let mut tmp = 0 as $t;

            let len = s.chars().count();

            for c in s.chars().into_iter().skip(1).take(len - 2) {
                 if c == ' ' || c == '\n' || c == '\t'{
                    if (c == '\n'  || c == '\t') && concating {
                        panic!("incorrect array, file: {}", path);
                    }
                    continue;
                } else if c == '_' {
                    if concating {
                        continue;
                    } else {
                        panic!("incorrect array, file: {}", path);
                    }
                } else if c == ',' {
                    if concating {
                        if concating_ignore {
                            if stringify!($t).ne(concating_ignore_s.as_str()) {
                                panic!("incorrect array, file: {}", path);
                            }
                        }

                        result[p] = tmp;
                        p += 1;

                        concating = false;
                        concating_ignore = false;
                        concating_ignore_s.clear();

                        tmp = 0 as $t;
                    } else {
                        panic!("incorrect array, file: {}", path);
                    }
                } else if c == '-' {
                    panic!("incorrect array, file: {}", path);
                } else if concating_ignore {
                    concating_ignore_s.push(c);
                } else if c >= '0' && c <= '9'{
                    let mut n = ((c as u32) - ('0' as u32)) as $t;

                    tmp = tmp * (10 as $t) + n;

                    concating = true;
                } else if c == '.' {
                    panic!("incorrect array, file: {}", path);
                } else {
                    if concating {
                        concating_ignore = true;
                        concating_ignore_s.push(c);
                    } else {
                        panic!("incorrect array, file: {}", path);
                    }
                }
            }

            if concating {
                if concating_ignore {
                    if stringify!($t).ne(concating_ignore_s.as_str()) {
                        panic!("incorrect array, file: {}", path);
                    }
                }

                result[p] = tmp;
                p += 1;
            }

            if p != $s {
                panic!("incorrect array, file: {}", path);
            }

            result
        }
    }
}

#[cfg(debug_assertions)]
#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_array_inner_if {
    ( $name:ident: [$t:ident; $s:expr], $path:expr ) => {
        {
            use ::std::fs::File;
            use ::std::io::Read;

            let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", $path);

            let v = {
                let mut f = File::open(&path).unwrap();

                let mut v: Vec<u8> = Vec::new();

                f.read_to_end(&mut v).unwrap();

                v
            };

            let s = String::from_utf8(v).unwrap();

            let s = s.trim();

            let len = s.len();

            if len < 2 || &s[0..1] != "[" || &s[len - 1..len] != "]" {
                panic!("incorrect array, file: {}", path);
            }

            let mut result = [0 as $t; $s];

            let mut p = 0usize;

            let mut concating = false;
            let mut negative = false;
            let mut concating_ignore = false;
            let mut concating_ignore_s = String::new();
            let mut floating = 0usize;
            let mut tmp = 0 as $t;

            let len = s.chars().count();

            for c in s.chars().into_iter().skip(1).take(len - 2) {
                 if c == ' ' || c == '\n'  || c == '\t'{
                    if (c == '\n'  || c == '\t') && concating {
                        panic!("incorrect array, file: {}", path);
                    }
                    continue;
                } else if c == '_' {
                    if concating {
                        continue;
                    } else {
                        panic!("incorrect array, file: {}", path);
                    }
                } else if c == ',' {
                    if concating {
                        if concating_ignore {
                            if stringify!($t).ne(concating_ignore_s.as_str()) {
                                panic!("incorrect array, file: {}", path);
                            }
                        } else {
                            if floating == 0 && stringify!($t).starts_with("f") {
                                panic!("incorrect array, file: {}", path);
                            }
                        }

                        if negative {
                            result[p] = -tmp;
                        } else {
                            result[p] = tmp;
                        }
                        p += 1;

                        concating = false;
                        negative = false;
                        concating_ignore = false;
                        concating_ignore_s.clear();

                        floating = 0;

                        tmp = 0 as $t;
                    } else {
                        panic!("incorrect array, file: {}", path);
                    }
                } else if c == '-' {
                    if concating {
                        panic!("incorrect array, file: {}", path);
                    } else {
                        negative = true;
                    }
                } else if concating_ignore {
                    concating_ignore_s.push(c);
                } else if c >= '0' && c <= '9'{
                    let mut n = ((c as u32) - ('0' as u32)) as $t;

                    if floating > 0 {
                        for _ in 0..floating {
                            n /= 10 as $t;
                        }

                        floating += 1;

                        tmp = tmp + n;
                    } else {
                        tmp = tmp * (10 as $t) + n;
                    }

                    concating = true;
                } else if c == '.' {
                    floating = 1;
                } else {
                    if concating {
                        concating_ignore = true;
                        concating_ignore_s.push(c);
                    } else {
                        panic!("incorrect array, file: {}", path);
                    }
                }
            }

            if concating {
                if concating_ignore {
                    if stringify!($t).ne(concating_ignore_s.as_str()) {
                        panic!("incorrect array, file: {}", path);
                    }
                } else {
                    if floating == 0 && stringify!($t).starts_with("f") {
                        panic!("incorrect array, file: {}", path);
                    }
                }

                if negative {
                    result[p] = -tmp;
                } else {
                    result[p] = tmp;
                }
                p += 1;
            } else {
                if negative {
                    panic!("incorrect array, file: {}", path);
                }
            }

            if p != $s {
                panic!("incorrect array, file: {}", path);
            }

            result
        }
    }
}

#[cfg(debug_assertions)]
#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_array_inner {
    ( $name:ident: [i8; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_if!($name: [i8; $s], $path)
        }
    };
    ( $name:ident: [i16; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_if!($name: [i16; $s], $path)
        }
    };
    ( $name:ident: [i32; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_if!($name: [i32; $s], $path)
        }
    };
    ( $name:ident: [i64; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_if!($name: [i64; $s], $path)
        }
    };
    ( $name:ident: [i128; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_if!($name: [i128; $s], $path)
        }
    };
    ( $name:ident: [f32; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_if!($name: [f32; $s], $path)
        }
    };
    ( $name:ident: [f64; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_if!($name: [f64; $s], $path)
        }
    };
    ( $name:ident: [u8; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_u!($name: [u8; $s], $path)
        }
    };
    ( $name:ident: [u16; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_u!($name: [u16; $s], $path)
        }
    };
    ( $name:ident: [u32; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_u!($name: [u32; $s], $path)
        }
    };
    ( $name:ident: [u64; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_u!($name: [u64; $s], $path)
        }
    };
    ( $name:ident: [u128; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_u!($name: [u128; $s], $path)
        }
    };
    ( $name:ident: [char; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_c!($name: [char; $s], $path)
        }
    };
    ( $name:ident: [bool; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_b!($name: [bool; $s], $path)
        }
    };
    ( $name:ident: [&'static str; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_s!($name: [&'static str; $s], $path)
        }
    };
    ( $name:ident: [&'static str; $s:expr], $path:expr, $($paths:expr), + ) => {
        {
            let mut v: Vec<[&'static str; $s]> = Vec::with_capacity(lazy_static_include_counter!($path $(, $paths)+));

            v.push(lazy_static_include_array_inner!($name: [&'static str; $s], $path));

            $(
                v.push(lazy_static_include_array_inner!($name: [&'static str; $s], $paths));
            )+

            v
        }
    };
    ( $name:ident: [$t:ident; $s:expr], $path:expr, $($paths:expr), + ) => {
        {
            let mut v: Vec<[$t; $s]> = Vec::with_capacity(lazy_static_include_counter!($path $(, $paths)+));

            v.push(lazy_static_include_array_inner!($name: [$t; $s], $path));

            $(
                v.push(lazy_static_include_array_inner!($name: [$t; $s], $paths));
            )+

            v
        }
    };
}

#[macro_export]
macro_rules! lazy_static_include_array {
    ( $name:ident: [&'static str; $s:expr], $path:expr ) => {
        lazy_static! {
            static ref $name: [&'static str; $s] = lazy_static_include_array_inner!($name: [&'static str; $s], $path);
        }

        lazy_static_include_array_impl!($name);
    };
    ( $name:ident: [&'static str; $s:expr], $path:expr, $($paths:expr), + ) => {
        lazy_static! {
            static ref $name: Vec<[&'static str; $s]> = lazy_static_include_array_inner!($name: [&'static str; $s], $path $(, $paths)+);
        }

        lazy_static_include_array_impl!($name);
    };
    ( pub $name:ident: [&'static str; $s:expr], $path:expr ) => {
        lazy_static! {
            pub static ref $name: [&'static str; $s] = lazy_static_include_array_inner!($name: [&'static str; $s], $path);
        }

        lazy_static_include_array_impl!($name);
    };
    ( pub $name:ident: [&'static str; $s:expr], $path:expr, $($paths:expr), + ) => {
        lazy_static! {
            pub static ref $name: Vec<[&'static str; $s]> = lazy_static_include_array_inner!($name: [&'static str; $s], $path $(, $paths)+);
        }

        lazy_static_include_array_impl!($name);
    };
    ( $name:ident: [$t:ident; $s:expr], $path:expr ) => {
        lazy_static! {
            static ref $name: [$t; $s] = lazy_static_include_array_inner!($name: [$t; $s], $path);
        }

        lazy_static_include_array_impl!($name);
    };
    ( $name:ident: [$t:ident; $s:expr], $path:expr, $($paths:expr), + ) => {
        lazy_static! {
            static ref $name: Vec<[$t; $s]> = lazy_static_include_array_inner!($name: [$t; $s], $path $(, $paths)+);
        }

        lazy_static_include_array_impl!($name);
    };
    ( pub $name:ident: [$t:ident; $s:expr], $path:expr ) => {
        lazy_static! {
            pub static ref $name: [$t; $s] = lazy_static_include_array_inner!($name: [$t; $s], $path);
        }

        lazy_static_include_array_impl!($name);
    };
    ( pub $name:ident: [$t:ident; $s:expr], $path:expr, $($paths:expr), + ) => {
        lazy_static! {
            pub static ref $name: Vec<[$t; $s]> = lazy_static_include_array_inner!($name: [$t; $s], $path $(, $paths)+);
        }

        lazy_static_include_array_impl!($name);
    };
}

// TODO -----include_array END-----