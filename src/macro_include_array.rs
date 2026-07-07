#[cfg(debug_assertions)]
/// Includes a file containing a rust array.
///
/// The file is located relative to the directory containing the manifest of your package.
#[macro_export]
macro_rules! lazy_static_include_array {
    ( @i [$t:ident; $s:expr], $path:expr ) => {
        {
            let path = $crate::manifest_dir_macros::not_directory_path!($path);

            let text = ::std::fs::read_to_string(path).unwrap();

            let s = text.trim();

            let mut result = [0 as $t; $s];

            if let Ok($crate::syn::Expr::Array(array)) = $crate::syn::parse_str(s) {
                for (i, l) in array.elems.into_iter().enumerate() {
                    if i >= $s {
                        panic!("incorrect length, bigger than {}, file: {}", $s, path);
                    }

                    let mut neg = false;

                    let exp = match l {
                        $crate::syn::Expr::Lit(exp) => exp,
                        $crate::syn::Expr::Unary(exp) => {
                            neg = true;

                            match exp.expr.as_ref() {
                                $crate::syn::Expr::Lit(exp) => exp.clone(),
                                _ => {
                                    panic!("incorrect element type, index = {}, file: {}", i, path);
                                }
                            }
                        }
                        _ => {
                            panic!("incorrect element type, index = {}, file: {}", i, path);
                        }
                    };

                    let accept_suffix = stringify!($t);

                    match exp.lit {
                        $crate::syn::Lit::Int(n) => {
                            let suffix = n.suffix();

                            if !suffix.is_empty() && suffix != accept_suffix {
                                panic!("incorrect element type, index = {}, file: {}", i, path);
                            }

                            let n: $t = n.base10_parse().unwrap();

                            result[i] = if neg {
                                -n
                            } else {
                                n
                            };
                        }
                        _ => {
                            panic!("incorrect element type, index = {}, file: {}", i, path);
                        }
                    }
                }

                result
            } else {
                panic!("incorrect array, file: {}", path);
            }
        }
    };
    ( @u [$t:ident; $s:expr], $path:expr ) => {
        {
            let path = $crate::manifest_dir_macros::not_directory_path!($path);

            let text = ::std::fs::read_to_string(path).unwrap();

            let s = text.trim();

            let mut result = [0 as $t; $s];

            if let Ok($crate::syn::Expr::Array(array)) = $crate::syn::parse_str(s) {
                for (i, l) in array.elems.into_iter().enumerate() {
                    if i >= $s {
                        panic!("incorrect length, bigger than {}, file: {}", $s, path);
                    }

                    let exp = match l {
                        $crate::syn::Expr::Lit(exp) => exp,
                        _ => {
                            panic!("incorrect element type, index = {}, file: {}", i, path);
                        }
                    };

                    let accept_suffix = stringify!($t);

                    match exp.lit {
                        $crate::syn::Lit::Int(n) => {
                            let suffix = n.suffix();

                            if !suffix.is_empty() && suffix != accept_suffix {
                                panic!("incorrect element type, index = {}, file: {}", i, path);
                            }

                            result[i] = n.base10_parse().unwrap();
                        }
                        _ => {
                            panic!("incorrect element type, index = {}, file: {}", i, path);
                        }
                    }
                }

                result
            } else {
                panic!("incorrect array, file: {}", path);
            }
        }
    };
    ( @f [$t:ident; $s:expr], $path:expr ) => {
        {
            let path = $crate::manifest_dir_macros::not_directory_path!($path);

            let text = ::std::fs::read_to_string(path).unwrap();

            let s = text.trim();

            let mut result = [0 as $t; $s];

            if let Ok($crate::syn::Expr::Array(array)) = $crate::syn::parse_str(s) {
                for (i, l) in array.elems.into_iter().enumerate() {
                    if i >= $s {
                        panic!("incorrect length, bigger than {}, file: {}", $s, path);
                    }

                    let mut neg = false;

                    let exp = match l {
                        $crate::syn::Expr::Lit(exp) => exp,
                        $crate::syn::Expr::Unary(exp) => {
                            neg = true;

                            match exp.expr.as_ref() {
                                $crate::syn::Expr::Lit(exp) => exp.clone(),
                                _ => {
                                    panic!("incorrect element type, index = {}, file: {}", i, path);
                                }
                            }
                        }
                        _ => {
                            panic!("incorrect element type, index = {}, file: {}", i, path);
                        }
                    };

                    let accept_suffix = stringify!($t);

                    match exp.lit {
                        $crate::syn::Lit::Float(f) => {
                            let suffix = f.suffix();

                            if !suffix.is_empty() && suffix != accept_suffix {
                                panic!("incorrect element type, index = {}, file: {}", i, path);
                            }

                            let f: $t = f.base10_parse().unwrap();

                            result[i] = if neg {
                                -f
                            } else {
                                f
                            };
                        }
                        $crate::syn::Lit::Int(n) => {
                            let suffix = n.suffix();

                            if suffix != accept_suffix {
                                panic!("incorrect element type, index = {}, file: {}", i, path);
                            }

                            let n: $t = n.base10_parse().unwrap();

                            result[i] = if neg {
                                -n
                            } else {
                                n
                            };
                        }
                        _ => {
                            panic!("incorrect element type, index = {}, file: {}", i, path);
                        }
                    }
                }

                result
            } else {
                panic!("incorrect array, file: {}", path);
            }
        }
    };
    ( @c [$t:ident; $s:expr], $path:expr ) => {
        {
            let path = $crate::manifest_dir_macros::not_directory_path!($path);

            let text = ::std::fs::read_to_string(path).unwrap();

            let s = text.trim();

            let mut result = ['\0'; $s];

            if let Ok($crate::syn::Expr::Array(array)) = $crate::syn::parse_str(s) {
                for (i, l) in array.elems.into_iter().enumerate() {
                    if i >= $s {
                        panic!("incorrect length, bigger than {}, file: {}", $s, path);
                    }

                    if let $crate::syn::Expr::Lit(exp) = l {
                        match exp.lit {
                            $crate::syn::Lit::Char(c) => {
                                result[i] = c.value();
                            }
                            _ => {
                                panic!("incorrect element type, index = {}, file: {}", i, path);
                            }
                        }
                    } else {
                        panic!("incorrect element type, index = {}, file: {}", i, path);
                    }
                }

                result
            } else {
                panic!("incorrect array, file: {}", path);
            }
        }
    };
    ( @b [$t:ident; $s:expr], $path:expr ) => {
        {
            let path = $crate::manifest_dir_macros::not_directory_path!($path);

            let text = ::std::fs::read_to_string(path).unwrap();

            let s = text.trim();

            let mut result = [false; $s];

            if let Ok($crate::syn::Expr::Array(array)) = $crate::syn::parse_str(s) {
                for (i, l) in array.elems.into_iter().enumerate() {
                    if i >= $s {
                        panic!("incorrect length, bigger than {}, file: {}", $s, path);
                    }

                    if let $crate::syn::Expr::Lit(exp) = l {
                        match exp.lit {
                            $crate::syn::Lit::Bool(b) => {
                                result[i] = b.value;
                            }
                            _ => {
                                panic!("incorrect element type, index = {}, file: {}", i, path);
                            }
                        }
                    } else {
                        panic!("incorrect element type, index = {}, file: {}", i, path);
                    }
                }

                result
            } else {
                panic!("incorrect array, file: {}", path);
            }
        }
    };
    ( @s [$s:expr], $path:expr ) => {
        {
            let path = $crate::manifest_dir_macros::not_directory_path!($path);

            let text = ::std::fs::read_to_string(path).unwrap();

            let s = text.trim();

            let mut result: [&'static str; $s] = [""; $s];

            if let Ok($crate::syn::Expr::Array(array)) = $crate::syn::parse_str(s) {
                for (i, l) in array.elems.into_iter().enumerate() {
                    if i >= $s {
                        panic!("incorrect length, bigger than {}, file: {}", $s, path);
                    }

                    if let $crate::syn::Expr::Lit(exp) = l {
                        match exp.lit {
                            $crate::syn::Lit::Str(s) => {
                                // Leak each string to get a `&'static str` reference, because the data needs to live as long as the program anyway.
                                result[i] = s.value().leak();
                            }
                            _ => {
                                panic!("incorrect element type, index = {}, file: {}", i, path);
                            }
                        }
                    } else {
                        panic!("incorrect element type, index = {}, file: {}", i, path);
                    }
                }

                result
            } else {
                panic!("incorrect array, file: {}", path);
            }
        }
    };
    ( @type [isize; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@i [isize; $s], $path)
    };
    ( @type [i8; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@i [i8; $s], $path)
    };
    ( @type [i16; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@i [i16; $s], $path)
    };
    ( @type [i32; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@i [i32; $s], $path)
    };
    ( @type [i64; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@i [i64; $s], $path)
    };
    ( @type [i128; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@i [i128; $s], $path)
    };
    ( @type [usize; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@u [usize; $s], $path)
    };
    ( @type [u8; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@u [u8; $s], $path)
    };
    ( @type [u16; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@u [u16; $s], $path)
    };
    ( @type [u32; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@u [u32; $s], $path)
    };
    ( @type [u64; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@u [u64; $s], $path)
    };
    ( @type [u128; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@u [u128; $s], $path)
    };
    ( @type [f32; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@f [f32; $s], $path)
    };
    ( @type [f64; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@f [f64; $s], $path)
    };
    ( @type [char; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@c [char; $s], $path)
    };
    ( @type [bool; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@b [bool; $s], $path)
    };
    ( @type [&'static str; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@s [$s], $path)
    };
    ( @unit $(#[$attr: meta])* $name:ident: [$(& $lt:lifetime)? $t:ident; $s:expr] => $path:expr ) => {
        $(#[$attr])*
        static $name: ::std::sync::LazyLock<[$(& $lt)? $t; $s]> = ::std::sync::LazyLock::new(|| $crate::lazy_static_include_array!(@type [$(& $lt)? $t; $s], $path));
    };
    ( @unit $(#[$attr: meta])* pub$(($($v:tt)+))? $name:ident: [$(& $lt:lifetime)? $t:ident; $s:expr] => $path:expr ) => {
        $(#[$attr])*
        pub$(($($v)+))? static $name: ::std::sync::LazyLock<[$(& $lt)? $t; $s]> = ::std::sync::LazyLock::new(|| $crate::lazy_static_include_array!(@type [$(& $lt)? $t; $s], $path));
    };
    ( $($(#[$attr: meta])* $name:ident: [$(& $lt:lifetime)? $t:ident; $s:expr] => $path:expr),* $(,)* ) => {
        $(
            $crate::lazy_static_include_array! {
                @unit
                $(#[$attr])*
                $name: [$(& $lt)? $t; $s] => $path
            }
        )*
    };
    ( $($(#[$attr: meta])* pub$(($($v:tt)+))? $name:ident: [$(& $lt:lifetime)? $t:ident; $s:expr] => $path:expr),* $(,)* ) => {
        $(
            $crate::lazy_static_include_array! {
                @unit
                $(#[$attr])*
                pub$(($($v)+))? $name: [$(& $lt)? $t; $s] => $path
            }
        )*
    };
}

#[cfg(not(debug_assertions))]
/// Includes a file containing a rust array.
///
/// The file is located relative to the directory containing the manifest of your package.
#[macro_export]
macro_rules! lazy_static_include_array {
    ( @unit $(#[$attr: meta])* $name:ident: [$(& $lt:lifetime)? $t:ident; $s:expr] => $path:expr ) => {
        $(#[$attr])*
        static $name: ::std::sync::LazyLock<[$(& $lt)? $t; $s]> = ::std::sync::LazyLock::new(|| include!($crate::manifest_dir_macros::path!($path)));
    };
    ( @unit $(#[$attr: meta])* pub$(($($v:tt)+))? $name:ident: [$(& $lt:lifetime)? $t:ident; $s:expr] => $path:expr ) => {
        $(#[$attr])*
        pub$(($($v)+))? static $name: ::std::sync::LazyLock<[$(& $lt)? $t; $s]> = ::std::sync::LazyLock::new(|| include!($crate::manifest_dir_macros::path!($path)));
    };
    ( $($(#[$attr: meta])* $name:ident: [$(& $lt:lifetime)? $t:ident; $s:expr] => $path:expr),* $(,)* ) => {
        $(
            $crate::lazy_static_include_array! {
                @unit
                $(#[$attr])*
                $name: [$(& $lt)? $t; $s] => $path
            }
        )*
    };
    ( $($(#[$attr: meta])* pub$(($($v:tt)+))? $name:ident: [$(& $lt:lifetime)? $t:ident; $s:expr] => $path:expr),* $(,)* ) => {
        $(
            $crate::lazy_static_include_array! {
                @unit
                $(#[$attr])*
                pub$(($($v)+))? $name: [$(& $lt)? $t; $s] => $path
            }
        )*
    };
}
