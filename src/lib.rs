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
//! You should notice that the struct created from `include_bytes` and `include_str` macros isn't equal to `&'static [u8]` or `&'static str`.
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