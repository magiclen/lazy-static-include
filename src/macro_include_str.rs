#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_str_impl {
    ($name:ident) => {
        impl ::std::cmp::PartialEq<str> for $name {
            fn eq(&self, other: &str) -> bool {
                (*$name).eq(other)
            }
        }

        impl<'a> ::std::cmp::PartialEq<&'a str> for $name {
            fn eq(&self, other: &&'a str) -> bool {
                (&*$name).eq(other)
            }
        }

        impl ::std::cmp::PartialEq for $name {
            fn eq(&self, other: &$name) -> bool {
                true
            }
        }

        impl<'a> ::std::cmp::PartialEq<$name> for &'a str {
            fn eq(&self, other: &$name) -> bool {
                self.eq(&*$name)
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

        impl<T: ?Sized> AsRef<T> for $name
        where
            str: ::std::convert::AsRef<T>,
        {
            fn as_ref(&self) -> &T {
                (*$name).as_ref()
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_str_multiple_impl {
    ($name:ident) => {
        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Debug::fmt(&*$name, f)
            }
        }
    };
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
    ( $name:ident, Vec, $($paths:expr), + ) => {
        {
            let mut v: Vec<&'static str> = Vec::with_capacity(lazy_static_include_counter!(Vec $(, $paths)+));

            $(
                v.push(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $paths)));
            )+

            v
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
    ( $name:ident, Vec, $($paths:expr), + ) => {
        {
            use ::std::fs::File;
            use ::std::io::Read;
            use ::std::mem;

            let mut v: Vec<&'static str> = Vec::with_capacity(lazy_static_include_counter!(Vec $(, $paths)+));

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
    ( $name:ident, $path:expr $(,)* ) => {
        lazy_static! {
            static ref $name: &'static str = lazy_static_include_str_inner!($name, $path);
        }

        lazy_static_include_str_impl!($name);
    };
    ( $name:ident, Vec, $($paths:expr), + $(,)* ) => {
        lazy_static! {
            static ref $name: Vec<&'static str> = lazy_static_include_str_inner!($name, Vec $(, $paths)+);
        }

        lazy_static_include_str_multiple_impl!($name);
    };
    ( $name:ident, $path:expr, $($paths:expr), + $(,)* ) => {
        lazy_static! {
            static ref $name: Vec<&'static str> = lazy_static_include_str_inner!($name, $path $(, $paths)+);
        }

        lazy_static_include_str_multiple_impl!($name);
    };
    ( pub $name:ident, $path:expr $(,)* ) => {
        lazy_static! {
            pub static ref $name: &'static str = lazy_static_include_str_inner!($name, $path);
        }

        lazy_static_include_str_impl!($name);
    };
    ( pub($($vis:tt)*) $name:ident, $path:expr $(,)* ) => {
        lazy_static! {
            pub($($vis)*) static ref $name: &'static str = lazy_static_include_str_inner!($name, $path);
        }

        lazy_static_include_str_impl!($name);
    };
    ( pub $name:ident, Vec, $($paths:expr), + $(,)* ) => {
        lazy_static! {
            pub static ref $name: Vec<&'static str> = lazy_static_include_str_inner!($name, Vec $(, $paths)+);
        }

        lazy_static_include_str_multiple_impl!($name);
    };
    ( pub($($vis:tt)*) $name:ident, Vec, $($paths:expr), + $(,)* ) => {
        lazy_static! {
            pub($($vis)*) static ref $name: Vec<&'static str> = lazy_static_include_str_inner!($name, Vec $(, $paths)+);
        }

        lazy_static_include_str_multiple_impl!($name);
    };
    ( pub $name:ident, $path:expr, $($paths:expr), + $(,)* ) => {
        lazy_static! {
            pub static ref $name: Vec<&'static str> = lazy_static_include_str_inner!($name, $path $(, $paths)+);
        }

        lazy_static_include_str_multiple_impl!($name);
    };
    ( pub($($vis:tt)*) $name:ident, $path:expr, $($paths:expr), + $(,)* ) => {
        lazy_static! {
            pub($($vis)*) static ref $name: Vec<&'static str> = lazy_static_include_str_inner!($name, $path $(, $paths)+);
        }

        lazy_static_include_str_multiple_impl!($name);
    };
}

#[macro_export]
macro_rules! lazy_static_include_str_vec {
    ( $name:ident, $($paths:expr), + $(,)* ) => {
        lazy_static! {
            static ref $name: Vec<&'static str> = lazy_static_include_str_inner!($name, Vec $(, $paths)+);
        }

        lazy_static_include_str_multiple_impl!($name);
    };
    ( pub $name:ident, $($paths:expr), + $(,)* ) => {
        lazy_static! {
            pub static ref $name: Vec<&'static str> = lazy_static_include_str_inner!($name, Vec $(, $paths)+);
        }

        lazy_static_include_str_multiple_impl!($name);
    };
    ( pub($($vis:tt)*) $name:ident, $($paths:expr), + $(,)* ) => {
        lazy_static! {
            pub static ref $name: Vec<&'static str> = lazy_static_include_str_inner!($name, Vec $(, $paths)+);
        }

        lazy_static_include_str_multiple_impl!($name);
    };
}
