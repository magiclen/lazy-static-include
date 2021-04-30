#[cfg(debug_assertions)]
/// Includes a utf8-encoded file as a string slice (`&'static str`).
///
/// The file is located relative to the directory containing the manifest of your package.
#[macro_export]
macro_rules! lazy_static_include_str {
    ( @impl $name:ident ) => {
        impl ::std::cmp::PartialEq<str> for $name {
            #[inline]
            fn eq(&self, other: &str) -> bool {
                (*$name).eq(other)
            }
        }

        impl<'a> ::std::cmp::PartialEq<&'a str> for $name {
            #[inline]
            fn eq(&self, other: &&'a str) -> bool {
                (&*$name).eq(other)
            }
        }

        impl ::std::cmp::PartialEq for $name {
            #[inline]
            fn eq(&self, other: &$name) -> bool {
                true
            }
        }

        impl<'a> ::std::cmp::PartialEq<$name> for &'a str {
            #[inline]
            fn eq(&self, other: &$name) -> bool {
                self.eq(&*$name)
            }
        }

        impl ::std::fmt::Debug for $name {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Debug::fmt(*$name, f)
            }
        }

        impl ::std::fmt::Display for $name {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(*$name, f)
            }
        }

        impl<T> ::std::convert::AsRef<T> for $name
        where
            T: ?Sized,
            str: ::std::convert::AsRef<T>,
        {
            #[inline]
            fn as_ref(&self) -> &T {
                (*$name).as_ref()
            }
        }
    };
    ( @inner $name:ident, $path:expr ) => {
        {
            use ::std::fs;
            use ::std::mem::{forget, transmute};

            let path = $crate::slash_formatter::concat_with_file_separator!(env!("CARGO_MANIFEST_DIR"), $path);

            let text = fs::read_to_string(path).unwrap();

            unsafe {
                let ret = transmute(text.as_str());
                forget(text);
                ret
            }
        }
    };
    ( @unit $(#[$attr: meta])* ($v:tt) $name:ident => $path:expr ) => {
        $crate::lazy_static::lazy_static! {
            $(#[$attr])*
            static ref $name: &'static str = $crate::lazy_static_include_str!(@inner $name, $path);
        }

        $crate::lazy_static_include_str!(@impl $name);
    };
    ( @unit $(#[$attr: meta])* (pub$(($($v:tt)+))?) $name:ident => $path:expr ) => {
        $crate::lazy_static::lazy_static! {
            $(#[$attr])*
            pub$(($($v)+))? static ref $name: &'static str = $crate::lazy_static_include_str!(@inner $name, $path);
        }

        $crate::lazy_static_include_str!(@impl $name);
    };
    ( $($(#[$attr: meta])* $v:vis $name:ident => $path:expr),* $(,)* ) => {
        $(
            $crate::lazy_static_include_str! {
                @unit
                $(#[$attr])*
                ($v) $name => $path
            }
        )*
    };
}

#[cfg(not(debug_assertions))]
/// Includes a utf8-encoded file as a string slice (`&'static str`).
///
/// The file is located relative to the directory containing the manifest of your package.
#[macro_export]
macro_rules! lazy_static_include_str {
    ( @impl $name:ident ) => {
        impl ::std::cmp::PartialEq<str> for $name {
            #[inline]
            fn eq(&self, other: &str) -> bool {
                (*$name).eq(other)
            }
        }

        impl<'a> ::std::cmp::PartialEq<&'a str> for $name {
            #[inline]
            fn eq(&self, other: &&'a str) -> bool {
                (&*$name).eq(other)
            }
        }

        impl ::std::cmp::PartialEq for $name {
            #[inline]
            fn eq(&self, other: &$name) -> bool {
                true
            }
        }

        impl<'a> ::std::cmp::PartialEq<$name> for &'a str {
            #[inline]
            fn eq(&self, other: &$name) -> bool {
                self.eq(&*$name)
            }
        }

        impl ::std::fmt::Debug for $name {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Debug::fmt(*$name, f)
            }
        }

        impl ::std::fmt::Display for $name {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(*$name, f)
            }
        }

        impl<T> ::std::convert::AsRef<T> for $name
        where
            T: ?Sized,
            str: ::std::convert::AsRef<T>,
        {
            #[inline]
            fn as_ref(&self) -> &T {
                (*$name).as_ref()
            }
        }
    };
    ( @unit $(#[$attr: meta])* ($v:tt) $name:ident => $path:expr ) => {
        $crate::lazy_static::lazy_static! {
            $(#[$attr])*
            static ref $name: &'static str = include_str!($crate::slash_formatter::concat_with_file_separator_debug_release!(env!("CARGO_MANIFEST_DIR"), $path));
        }

        $crate::lazy_static_include_str!(@impl $name);
    };
    ( @unit $(#[$attr: meta])* (pub$(($($v:tt)+))?) $name:ident => $path:expr ) => {
        $crate::lazy_static::lazy_static! {
            $(#[$attr])*
            pub$(($($v)+))? static ref $name: &'static str = include_str!($crate::slash_formatter::concat_with_file_separator_debug_release!(env!("CARGO_MANIFEST_DIR"), $path));
        }

        $crate::lazy_static_include_str!(@impl $name);
    };
    ( $($(#[$attr: meta])* $v:vis $name:ident => $path:expr),* $(,)* ) => {
        $(
            $crate::lazy_static_include_str! {
                @unit
                $(#[$attr])*
                ($v) $name => $path
            }
        )*
    };
}
