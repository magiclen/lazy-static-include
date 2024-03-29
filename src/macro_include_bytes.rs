#[cfg(debug_assertions)]
/// Includes a file as a reference to a byte array (`&'static [u8]`).
///
/// The file is located relative to the directory containing the manifest of your package.
#[macro_export]
macro_rules! lazy_static_include_bytes {
    ( @impl $name:ident ) => {
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

        impl<T> ::std::convert::AsRef<T> for $name
        where
            T: ?Sized,
            [u8]: ::std::convert::AsRef<T>,
        {
            fn as_ref(&self) -> &T {
                (*$name).as_ref()
            }
        }
    };
    ( @inner $name:ident, $path:expr ) => {
        {
            use ::std::fs;
            use ::std::mem::{forget, transmute};

            let path = $crate::manifest_dir_macros::not_directory_path!($path);

            let data = fs::read(path).unwrap();

            unsafe {
                let ret = transmute(data.as_slice());
                forget(data);
                ret
            }
        }
    };
    ( @unit $(#[$attr: meta])* $name:ident => $path:expr ) => {
        $crate::lazy_static::lazy_static! {
            $(#[$attr])*
            static ref $name: &'static [u8] = $crate::lazy_static_include_bytes!(@inner $name, $path);
        }

        $crate::lazy_static_include_bytes!(@impl $name);
    };
    ( @unit $(#[$attr: meta])* pub$(($($v:tt)+))? $name:ident => $path:expr ) => {
        $crate::lazy_static::lazy_static! {
            $(#[$attr])*
            pub$(($($v)+))? static ref $name: &'static [u8] = $crate::lazy_static_include_bytes!(@inner $name, $path);
        }

        $crate::lazy_static_include_bytes!(@impl $name);
    };
    ( $($(#[$attr: meta])* $name:ident => $path:expr),* $(,)* ) => {
        $(
            $crate::lazy_static_include_bytes! {
                @unit
                $(#[$attr])*
                $name => $path
            }
        )*
    };
    ( $($(#[$attr: meta])* pub$(($($v:tt)+))? $name:ident => $path:expr),* $(,)* ) => {
        $(
            $crate::lazy_static_include_bytes! {
                @unit
                $(#[$attr])*
                pub$(($($v)+))? $name => $path
            }
        )*
    };
}

#[cfg(not(debug_assertions))]
/// Includes a file as a reference to a byte array (`&'static [u8]`).
///
/// The file is located relative to the directory containing the manifest of your package.
#[macro_export]
macro_rules! lazy_static_include_bytes {
    ( @impl $name:ident ) => {
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

        impl<T> ::std::convert::AsRef<T> for $name
        where
            T: ?Sized,
            [u8]: ::std::convert::AsRef<T>,
        {
            fn as_ref(&self) -> &T {
                (*$name).as_ref()
            }
        }
    };
    ( @unit $(#[$attr: meta])* $name:ident => $path:expr ) => {
        $crate::lazy_static::lazy_static! {
            $(#[$attr])*
            static ref $name: &'static [u8] = include_bytes!($crate::manifest_dir_macros::path!($path));
        }

        $crate::lazy_static_include_bytes!(@impl $name);
    };
    ( @unit $(#[$attr: meta])* pub$(($($v:tt)+))? $name:ident => $path:expr ) => {
        $crate::lazy_static::lazy_static! {
            $(#[$attr])*
            pub$(($($v)+))? static ref $name: &'static [u8] = include_bytes!($crate::manifest_dir_macros::path!($path));
        }

        $crate::lazy_static_include_bytes!(@impl $name);
    };
    ( $($(#[$attr: meta])* $name:ident => $path:expr),* $(,)* ) => {
        $(
            $crate::lazy_static_include_bytes! {
                @unit
                $(#[$attr])*
                $name => $path
            }
        )*
    };
    ( $($(#[$attr: meta])* pub$(($($v:tt)+))? $name:ident => $path:expr),* $(,)* ) => {
        $(
            $crate::lazy_static_include_bytes! {
                @unit
                $(#[$attr])*
                pub$(($($v)+))? $name => $path
            }
        )*
    };
}
