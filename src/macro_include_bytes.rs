#[cfg(debug_assertions)]
/// Includes a file as a reference to a byte array (`&'static [u8]`).
///
/// The file is located relative to the directory containing the manifest of your package.
#[macro_export]
macro_rules! lazy_static_include_bytes {
    ( @inner $path:expr ) => {
        {
            let path = $crate::manifest_dir_macros::not_directory_path!($path);

            // Leak the file content to get a `&'static [u8]` reference, because the data needs to live as long as the program anyway.
            let data: &'static [u8] = ::std::fs::read(path).unwrap().leak();

            data
        }
    };
    ( @unit $(#[$attr: meta])* $name:ident => $path:expr ) => {
        $(#[$attr])*
        static $name: ::std::sync::LazyLock<&'static [u8]> = ::std::sync::LazyLock::new(|| $crate::lazy_static_include_bytes!(@inner $path));
    };
    ( @unit $(#[$attr: meta])* pub$(($($v:tt)+))? $name:ident => $path:expr ) => {
        $(#[$attr])*
        pub$(($($v)+))? static $name: ::std::sync::LazyLock<&'static [u8]> = ::std::sync::LazyLock::new(|| $crate::lazy_static_include_bytes!(@inner $path));
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
    ( @unit $(#[$attr: meta])* $name:ident => $path:expr ) => {
        $(#[$attr])*
        static $name: ::std::sync::LazyLock<&'static [u8]> = ::std::sync::LazyLock::new(|| include_bytes!($crate::manifest_dir_macros::path!($path)));
    };
    ( @unit $(#[$attr: meta])* pub$(($($v:tt)+))? $name:ident => $path:expr ) => {
        $(#[$attr])*
        pub$(($($v)+))? static $name: ::std::sync::LazyLock<&'static [u8]> = ::std::sync::LazyLock::new(|| include_bytes!($crate::manifest_dir_macros::path!($path)));
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
