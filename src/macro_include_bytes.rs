#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_bytes_impl {
    ($name:ident) => {
        impl<'a> ::std::cmp::PartialEq<&'a [u8]> for $name {
            fn eq(&self, other: &&'a [u8]) -> bool {
                (&*$name).eq(other)
            }
        }

        impl ::std::cmp::PartialEq for $name {
            fn eq(&self, other: &$name) -> bool {
                true
            }
        }

        impl<'a> ::std::cmp::PartialEq<$name> for &'a [u8] {
            fn eq(&self, other: &$name) -> bool {
                self.eq(&*$name)
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
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_bytes_multiple_impl {
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
macro_rules! lazy_static_include_bytes_inner {
    ( $name:ident, $path:expr ) => {
        {
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path))
        }
    };
    ( $name:ident, Vec, $($paths:expr), + ) => {
        {
            let mut v: Vec<&'static [u8]> = Vec::with_capacity(lazy_static_include_counter!(Vec $(, $paths)+));

            $(
                v.push(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $paths)));
            )+

            v
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
    ( $name:ident, Vec, $($paths:expr), + ) => {
        {
            use ::std::fs::File;
            use ::std::io::Read;
            use ::std::mem;

            let mut v: Vec<&'static [u8]> = Vec::with_capacity(lazy_static_include_counter!(Vec $(, $paths)+));

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
    ( $name:ident, $path:expr $(,)* ) => {
        lazy_static! {
            static ref $name: &'static [u8] = lazy_static_include_bytes_inner!($name, $path);
        }

        lazy_static_include_bytes_impl!($name);
    };
    ( $name:ident, $path:expr, $($paths:expr), + $(,)* ) => {
        lazy_static! {
            static ref $name: Vec<&'static [u8]> = lazy_static_include_bytes_inner!($name, $path $(, $paths)+);
        }

        lazy_static_include_bytes_multiple_impl!($name);
    };
    ( pub $name:ident, $path:expr $(,)* ) => {
        lazy_static! {
            pub static ref $name: &'static [u8] = lazy_static_include_bytes_inner!($name, $path);
        }

        lazy_static_include_bytes_impl!($name);
    };
    ( pub($($vis:tt)*) $name:ident, $path:expr $(,)* ) => {
        lazy_static! {
            pub($($vis)*) static ref $name: &'static [u8] = lazy_static_include_bytes_inner!($name, $path);
        }

        lazy_static_include_bytes_impl!($name);
    };
    ( pub $name:ident, $path:expr, $($paths:expr), + $(,)* ) => {
        lazy_static! {
            pub static ref $name: Vec<&'static [u8]> = lazy_static_include_bytes_inner!($name, $path $(, $paths)+);
        }

        lazy_static_include_bytes_multiple_impl!($name);
    };
    ( pub($($vis:tt)*) $name:ident, $path:expr, $($paths:expr), + $(,)* ) => {
        lazy_static! {
            pub($($vis)*) static ref $name: Vec<&'static [u8]> = lazy_static_include_bytes_inner!($name, $path $(, $paths)+);
        }

        lazy_static_include_bytes_multiple_impl!($name);
    };
}

#[macro_export]
macro_rules! lazy_static_include_bytes_vec {
    ( $name:ident, $($paths:expr), + $(,)* ) => {
        lazy_static! {
            static ref $name: Vec<&'static [u8]> = lazy_static_include_bytes_inner!($name, Vec $(, $paths)+);
        }

        lazy_static_include_bytes_multiple_impl!($name);
    };
    ( pub $name:ident, $($paths:expr), + $(,)* ) => {
        lazy_static! {
            pub static ref $name: Vec<&'static [u8]> = lazy_static_include_bytes_inner!($name, Vec $(, $paths)+);
        }

        lazy_static_include_bytes_multiple_impl!($name);
    };
    ( pub($($vis:tt)*) $name:ident, $($paths:expr), + $(,)* ) => {
        lazy_static! {
            pub($($vis)*) static ref $name: Vec<&'static [u8]> = lazy_static_include_bytes_inner!($name, Vec $(, $paths)+);
        }

        lazy_static_include_bytes_multiple_impl!($name);
    };
}
