#[cfg(debug_assertions)]
/// Includes a file containing a rust array.
///
/// The file is located relative to the directory containing the manifest of your package.
#[macro_export]
macro_rules! lazy_static_include_array {
    ( @i $name:ident: [$t:ident; $s:expr], $path:expr ) => {
        {
            let path = $crate::concat_with_file_separator!(env!("CARGO_MANIFEST_DIR"), $path);

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
    ( @u $name:ident: [$t:ident; $s:expr], $path:expr ) => {
        {
            let path = $crate::concat_with_file_separator!(env!("CARGO_MANIFEST_DIR"), $path);

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
    ( @f $name:ident: [$t:ident; $s:expr], $path:expr ) => {
        {
            let path = $crate::concat_with_file_separator!(env!("CARGO_MANIFEST_DIR"), $path);

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
    ( @c $name:ident: [$t:ident; $s:expr], $path:expr ) => {
        {
            let path = $crate::concat_with_file_separator!(env!("CARGO_MANIFEST_DIR"), $path);

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
    ( @b $name:ident: [$t:ident; $s:expr], $path:expr ) => {
        {
            let path = $crate::concat_with_file_separator!(env!("CARGO_MANIFEST_DIR"), $path);

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
    ( @s $name:ident: [$t:ident; $s:expr], $path:expr ) => {
        {
            use ::std::mem::{forget, transmute};

            let path = $crate::concat_with_file_separator!(env!("CARGO_MANIFEST_DIR"), $path);

            let text = ::std::fs::read_to_string(path).unwrap();

            let s = text.trim();

            let mut result = Vec::with_capacity($s);

            if let Ok($crate::syn::Expr::Array(array)) = $crate::syn::parse_str(s) {
                for (i, l) in array.elems.into_iter().enumerate() {
                    if i >= $s {
                        panic!("incorrect length, bigger than {}, file: {}", $s, path);
                    }

                    if let $crate::syn::Expr::Lit(exp) = l {
                        match exp.lit {
                            $crate::syn::Lit::Str(s) => {
                                result.push(s.value());
                            }
                            _ => {
                                panic!("incorrect element type, index = {}, file: {}", i, path);
                            }
                        }
                    } else {
                        panic!("incorrect element type, index = {}, file: {}", i, path);
                    }
                }

                let mut result_str = [""; $s];

                for (i, s) in result.iter().enumerate() {
                    result_str[i] = unsafe {
                        let ret = transmute(s.as_str());
                        ret
                    };
                }

                unsafe {
                    forget(result);
                };

                result_str
            } else {
                panic!("incorrect array, file: {}", path);
            }
        }
    };
    ( @type $name:ident: [isize; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@i $name: [isize; $s], $path);
    };
    ( @type $name:ident: [i8; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@i $name: [i8; $s], $path);
    };
    ( @type $name:ident: [i16; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@i $name: [i16; $s], $path);
    };
    ( @type $name:ident: [i32; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@i $name: [i32; $s], $path);
    };
    ( @type $name:ident: [i64; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@i $name: [i64; $s], $path);
    };
    ( @type $name:ident: [i128; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@i $name: [i128; $s], $path);
    };
    ( @type $name:ident: [usize; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@u $name: [usize; $s], $path);
    };
    ( @type $name:ident: [u8; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@u $name: [u8; $s], $path);
    };
    ( @type $name:ident: [u16; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@u $name: [u16; $s], $path);
    };
    ( @type $name:ident: [u32; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@u $name: [u32; $s], $path);
    };
    ( @type $name:ident: [u64; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@u $name: [u64; $s], $path);
    };
    ( @type $name:ident: [u128; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@u $name: [u128; $s], $path);
    };
    ( @type $name:ident: [f32; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@f $name: [f32; $s], $path);
    };
    ( @type $name:ident: [f64; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@f $name: [f64; $s], $path);
    };
    ( @type $name:ident: [char; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@c $name: [char; $s], $path);
    };
    ( @type $name:ident: [bool; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@b $name: [bool; $s], $path);
    };
    ( @type $name:ident: [&'static str; $s:expr], $path:expr ) => {
        $crate::lazy_static_include_array!(@s $name: [bool; $s], $path);
    };
    ( @unit $(#[$attr: meta])* ($v:tt) $name:ident: [$(& $lt:lifetime)? $t:ident; $s:expr] => $path:expr ) => {
        $crate::lazy_static! {
            $(#[$attr])*
            static ref $name: [$(& $lt)? $t; $s] = $crate::lazy_static_include_array!(@type $name: [$(& $lt)? $t; $s], $path);
        }
    };
    ( @unit $(#[$attr: meta])* (pub$(($($v:tt)+))?) $name:ident: [$(& $lt:lifetime)? $t:ident; $s:expr] => $path:expr ) => {
        $crate::lazy_static! {
            $(#[$attr])*
            pub$(($($v)+))? static ref $name: [$(& $lt)? $t; $s] = $crate::lazy_static_include_array!(@type $name: [$(& $lt)? $t; $s], $path);
        }
    };
    ( $($(#[$attr: meta])* $v:vis $name:ident: [$(& $lt:lifetime)? $t:ident; $s:expr] => $path:expr),* $(,)* ) => {
        $(
            $crate::lazy_static_include_array! {
                @unit
                $(#[$attr])*
                ($v) $name: [$(& $lt)? $t; $s] => $path
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
    ( @unit $(#[$attr: meta])* ($v:tt) $name:ident: [$(& $lt:lifetime)? $t:ident; $s:expr] => $path:expr ) => {
        $crate::lazy_static! {
            $(#[$attr])*
            static ref $name: [$(& $lt)? $t; $s] = include!($crate::concat_with_file_separator!(env!("CARGO_MANIFEST_DIR"), $path));
        }
    };
    ( @unit $(#[$attr: meta])* (pub$(($($v:tt)+))?) $name:ident: [$(& $lt:lifetime)? $t:ident; $s:expr] => $path:expr ) => {
        $crate::lazy_static! {
            $(#[$attr])*
            pub$(($($v)+))? static ref $name: [$(& $lt)? $t; $s] = include!($crate::concat_with_file_separator!(env!("CARGO_MANIFEST_DIR"), $path));
        }
    };
    ( $($(#[$attr: meta])* $v:vis $name:ident: [$(& $lt:lifetime)? $t:ident; $s:expr] => $path:expr),* $(,)* ) => {
        $(
            $crate::lazy_static_include_array! {
                @unit
                $(#[$attr])*
                ($v) $name: [$(& $lt)? $t; $s] => $path
            }
        )*
    };
}
