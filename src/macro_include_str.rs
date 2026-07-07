#[cfg(debug_assertions)]
/// Includes a utf8-encoded file as a string slice (`&'static str`).
///
/// The file is located relative to the directory containing the manifest of your package.
#[macro_export]
macro_rules! lazy_static_include_str {
    ( @inner $path:expr ) => {
        {
            let path = $crate::manifest_dir_macros::not_directory_path!($path);

            // Leak the file content to get a `&'static str` reference, because the data needs to live as long as the program anyway.
            let text: &'static str = ::std::fs::read_to_string(path).unwrap().leak();

            text
        }
    };
    ( @unit $(#[$attr: meta])* $name:ident => $path:expr ) => {
        $(#[$attr])*
        static $name: ::std::sync::LazyLock<&'static str> = ::std::sync::LazyLock::new(|| $crate::lazy_static_include_str!(@inner $path));
    };
    ( @unit $(#[$attr: meta])* pub$(($($v:tt)+))? $name:ident => $path:expr ) => {
        $(#[$attr])*
        pub$(($($v)+))? static $name: ::std::sync::LazyLock<&'static str> = ::std::sync::LazyLock::new(|| $crate::lazy_static_include_str!(@inner $path));
    };
    ( $($(#[$attr: meta])* $name:ident => $path:expr),* $(,)* ) => {
        $(
            $crate::lazy_static_include_str! {
                @unit
                $(#[$attr])*
                $name => $path
            }
        )*
    };
    ( $($(#[$attr: meta])* pub$(($($v:tt)+))? $name:ident => $path:expr),* $(,)* ) => {
        $(
            $crate::lazy_static_include_str! {
                @unit
                $(#[$attr])*
                pub$(($($v)+))? $name => $path
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
    ( @unit $(#[$attr: meta])* $name:ident => $path:expr ) => {
        $(#[$attr])*
        static $name: ::std::sync::LazyLock<&'static str> = ::std::sync::LazyLock::new(|| include_str!($crate::manifest_dir_macros::path!($path)));
    };
    ( @unit $(#[$attr: meta])* pub$(($($v:tt)+))? $name:ident => $path:expr ) => {
        $(#[$attr])*
        pub$(($($v)+))? static $name: ::std::sync::LazyLock<&'static str> = ::std::sync::LazyLock::new(|| include_str!($crate::manifest_dir_macros::path!($path)));
    };
    ( $($(#[$attr: meta])* $name:ident => $path:expr),* $(,)* ) => {
        $(
            $crate::lazy_static_include_str! {
                @unit
                $(#[$attr])*
                $name => $path
            }
        )*
    };
    ( $($(#[$attr: meta])* pub$(($($v:tt)+))? $name:ident => $path:expr),* $(,)* ) => {
        $(
            $crate::lazy_static_include_str! {
                @unit
                $(#[$attr])*
                pub$(($($v)+))? $name => $path
            }
        )*
    };
}
