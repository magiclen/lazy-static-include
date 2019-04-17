#[cfg(not(debug_assertions))]
#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_array_inner {
    ( $name:ident: [&'static str; $s:expr], $path:expr ) => {
        {
            include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path))
        }
    };
    ( $name:ident: [&'static str; $s:expr], Vec, $($paths:expr), + ) => {
        {
            let mut v: Vec<[&'static str; $s]> = Vec::with_capacity(lazy_static_include_counter!(Vec $(, $paths)+));

            $(
                v.push(include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $paths)));
            )+

            v
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
    ( $name:ident: [$t:ident; $s:expr], Vec, $($paths:expr), + ) => {
        {
            let mut v: Vec<[$t; $s]> = Vec::with_capacity(lazy_static_include_counter!(Vec $(, $paths)+));

            $(
                v.push(include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $paths)));
            )+

            v
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

#[cfg(all(debug_assertions, not(feature = "no_std")))]
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

            let mut result = [false; $s];

            if let Ok(::lazy_static_include::syn::Expr::Array(array)) = ::lazy_static_include::syn::parse_str(s) {
                for (i, l) in array.elems.into_iter().enumerate() {
                    if i >= $s {
                        panic!("incorrect length, bigger than {}, file: {}", $s, path);
                    }

                    if let ::lazy_static_include::syn::Expr::Lit(exp) = l {
                        match exp.lit {
                            ::lazy_static_include::syn::Lit::Bool(b) => {
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
    }
}

#[cfg(all(debug_assertions, not(feature = "no_std")))]
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

            let mut result = ['\0'; $s];

            if let Ok(::lazy_static_include::syn::Expr::Array(array)) = ::lazy_static_include::syn::parse_str(s) {
                for (i, l) in array.elems.into_iter().enumerate() {
                    if i >= $s {
                        panic!("incorrect length, bigger than {}, file: {}", $s, path);
                    }

                    if let ::lazy_static_include::syn::Expr::Lit(exp) = l {
                        match exp.lit {
                            ::lazy_static_include::syn::Lit::Char(c) => {
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
    }
}

#[cfg(all(debug_assertions, not(feature = "no_std")))]
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

            let mut result = Vec::with_capacity($s);

            if let Ok(::lazy_static_include::syn::Expr::Array(array)) = ::lazy_static_include::syn::parse_str(s) {
                for (i, l) in array.elems.into_iter().enumerate() {
                    if i >= $s {
                        panic!("incorrect length, bigger than {}, file: {}", $s, path);
                    }

                    if let ::lazy_static_include::syn::Expr::Lit(exp) = l {
                        match exp.lit {
                            ::lazy_static_include::syn::Lit::Str(s) => {
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
                        let ret = mem::transmute(s.as_str());
                        ret
                    };
                }

                unsafe {
                    mem::forget(result);
                };

                result_str
            } else {
                panic!("incorrect array, file: {}", path);
            }
        }
    }
}

#[cfg(all(debug_assertions, not(feature = "no_std")))]
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

            let mut result = [0 as $t; $s];

            if let Ok(::lazy_static_include::syn::Expr::Array(array)) = ::lazy_static_include::syn::parse_str(s) {
                for (i, l) in array.elems.into_iter().enumerate() {
                    if i >= $s {
                        panic!("incorrect length, bigger than {}, file: {}", $s, path);
                    }

                    if let ::lazy_static_include::syn::Expr::Lit(exp) = l {
                        match exp.lit {
                            ::lazy_static_include::syn::Lit::Int(u) => {
                                let accept_suffix = match stringify!($t) {
                                    "usize" => {
                                        ::lazy_static_include::syn::IntSuffix::Usize
                                    },
                                    "u8" => {
                                        ::lazy_static_include::syn::IntSuffix::U8
                                    },
                                    "u16" => {
                                        ::lazy_static_include::syn::IntSuffix::U16
                                    },
                                    "u32" => {
                                        ::lazy_static_include::syn::IntSuffix::U32
                                    },
                                    "u64" => {
                                        ::lazy_static_include::syn::IntSuffix::U64
                                    },
                                    _ => unreachable!()
                                };

                                let suffix = u.suffix();

                                if suffix != ::lazy_static_include::syn::IntSuffix::None && suffix != accept_suffix {
                                    panic!("incorrect element type, index = {}, file: {}", i, path);
                                }

                                let u = u.value();

                                if u > $t::max_value() as u64 {
                                    panic!("incorrect element, index = {}, bigger than {}, file: {}", i, $t::max_value(), path);
                                }

                                result[i] = u as $t;
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
    }
}

#[cfg(all(debug_assertions, not(feature = "no_std")))]
#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_array_inner_u128 {
    ( $name:ident: [$t:ident; $s:expr], $path:expr ) => {
        {
            use ::std::fs::File;
            use ::std::io::Read;
            use ::lazy_static_include::starts_ends_with_caseless::EndsWithCaseless;

            let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", $path);

            let v = {
                let mut f = File::open(&path).unwrap();

                let mut v: Vec<u8> = Vec::new();

                f.read_to_end(&mut v).unwrap();

                v
            };

            let s = String::from_utf8(v).unwrap();

            let s = s.trim();

            let mut result = [0 as $t; $s];

            if let Ok(::lazy_static_include::syn::Expr::Array(array)) = ::lazy_static_include::syn::parse_str(s) {
                for (i, l) in array.elems.into_iter().enumerate() {
                    if i >= $s {
                        panic!("incorrect length, bigger than {}, file: {}", $s, path);
                    }

                    if let ::lazy_static_include::syn::Expr::Lit(exp) = l {
                        match exp.lit {
                            ::lazy_static_include::syn::Lit::Verbatim(v) => {
                                let s = v.token.to_string();

                                let s = if s.ends_with_caseless_ascii("u128") {
                                    &s[..s.len() - 4]
                                } else {
                                    &s
                                };

                                let s = s.replace("_", "");

                                let u: u128 = s.parse().unwrap();

                                result[i] = u;
                            }
                            ::lazy_static_include::syn::Lit::Int(u) => {
                                let suffix = u.suffix();

                                if suffix != ::lazy_static_include::syn::IntSuffix::None && suffix != ::lazy_static_include::syn::IntSuffix::U128 {
                                    panic!("incorrect element type, index = {}, file: {}", i, path);
                                }

                                result[i] = u.value() as u128;
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
    }
}

#[cfg(all(debug_assertions, not(feature = "no_std")))]
#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_array_inner_i {
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

            let mut result = [0 as $t; $s];

            if let Ok(::lazy_static_include::syn::Expr::Array(array)) = ::lazy_static_include::syn::parse_str(s) {
                for (i, l) in array.elems.into_iter().enumerate() {
                    if i >= $s {
                        panic!("incorrect length, bigger than {}, file: {}", $s, path);
                    }

                    let mut neg = false;

                    let exp = match l {
                        ::lazy_static_include::syn::Expr::Lit(exp) => {
                            exp
                        }
                        ::lazy_static_include::syn::Expr::Unary(exp) => {
                            neg = true;

                            match exp.expr.as_ref() {
                                ::lazy_static_include::syn::Expr::Lit(exp) => {
                                    exp.clone()
                                }
                                _ => {
                                    panic!("incorrect element type, index = {}, file: {}", i, path);
                                }
                            }
                        }
                        _ => {
                            panic!("incorrect element type, index = {}, file: {}", i, path);
                        }
                    };

                    match exp.lit {
                        ::lazy_static_include::syn::Lit::Int(n) => {
                            let accept_suffix = match stringify!($t) {
                                "isize" => {
                                    ::lazy_static_include::syn::IntSuffix::Isize
                                },
                                "i8" => {
                                    ::lazy_static_include::syn::IntSuffix::I8
                                },
                                "i16" => {
                                    ::lazy_static_include::syn::IntSuffix::I16
                                },
                                "i32" => {
                                    ::lazy_static_include::syn::IntSuffix::I32
                                },
                                "i64" => {
                                    ::lazy_static_include::syn::IntSuffix::I64
                                },
                                _ => unreachable!()
                            };

                            let suffix = n.suffix();

                            if suffix != ::lazy_static_include::syn::IntSuffix::None && suffix != accept_suffix {
                                panic!("incorrect element type, index = {}, file: {}", i, path);
                            }

                            let n = n.value();

                            if neg {
                                if -1 * (n as i128) < $t::min_value() as i128 {
                                    panic!("incorrect element, index = {}, smaller than {}, file: {}", i, $t::min_value(), path);
                                }

                                result[i] = -1 * (n as $t);
                            } else {
                                if n > $t::max_value() as u64 {
                                    panic!("incorrect element, index = {}, bigger than {}, file: {}", i, $t::max_value(), path);
                                }

                                result[i] = n as $t;
                            }
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
    }
}

#[cfg(all(debug_assertions, not(feature = "no_std")))]
#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_array_inner_i128 {
    ( $name:ident: [$t:ident; $s:expr], $path:expr ) => {
        {
            use ::std::fs::File;
            use ::std::io::Read;
            use ::lazy_static_include::starts_ends_with_caseless::EndsWithCaseless;

            let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", $path);

            let v = {
                let mut f = File::open(&path).unwrap();

                let mut v: Vec<u8> = Vec::new();

                f.read_to_end(&mut v).unwrap();

                v
            };

            let s = String::from_utf8(v).unwrap();

            let s = s.trim();

            let mut result = [0 as $t; $s];

            if let Ok(::lazy_static_include::syn::Expr::Array(array)) = ::lazy_static_include::syn::parse_str(s) {
                for (i, l) in array.elems.into_iter().enumerate() {
                    if i >= $s {
                        panic!("incorrect length, bigger than {}, file: {}", $s, path);
                    }

                    let mut neg = false;

                    let exp = match l {
                        ::lazy_static_include::syn::Expr::Lit(exp) => {
                            exp
                        }
                        ::lazy_static_include::syn::Expr::Unary(exp) => {
                            neg = true;

                            match exp.expr.as_ref() {
                                ::lazy_static_include::syn::Expr::Lit(exp) => {
                                    exp.clone()
                                }
                                _ => {
                                    panic!("incorrect element type, index = {}, file: {}", i, path);
                                }
                            }
                        }
                        _ => {
                            panic!("incorrect element type, index = {}, file: {}", i, path);
                        }
                    };

                    match exp.lit {
                        ::lazy_static_include::syn::Lit::Verbatim(v) => {
                            let s = v.token.to_string();

                            let s = if s.ends_with_caseless_ascii("i128") {
                                &s[..s.len() - 4]
                            } else {
                                &s
                            };

                            let s = s.replace("_", "");

                            let n: i128 = s.parse().unwrap();

                            result[i] = n;
                        }
                        ::lazy_static_include::syn::Lit::Int(n) => {
                            let suffix = n.suffix();

                            if suffix != ::lazy_static_include::syn::IntSuffix::None && suffix != ::lazy_static_include::syn::IntSuffix::I128 {
                                panic!("incorrect element type, index = {}, file: {}", i, path);
                            }

                            if neg {
                                result[i] = -1 * (n.value() as i128);
                            } else {
                                result[i] = n.value() as i128;
                            }
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
    }
}

#[cfg(all(debug_assertions, not(feature = "no_std")))]
#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_array_inner_f32 {
    ( $name:ident: [$t:ident; $s:expr], $path:expr ) => {
        {
            use ::std::fs::File;
            use ::std::io::Read;
            use ::lazy_static_include::syn::export::ToTokens;
            use ::lazy_static_include::starts_ends_with_caseless::EndsWithCaseless;

            let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", $path);

            let v = {
                let mut f = File::open(&path).unwrap();

                let mut v: Vec<u8> = Vec::new();

                f.read_to_end(&mut v).unwrap();

                v
            };

            let s = String::from_utf8(v).unwrap();

            let s = s.trim();

            let mut result = [0 as $t; $s];

            if let Ok(::lazy_static_include::syn::Expr::Array(array)) = ::lazy_static_include::syn::parse_str(s) {
                for (i, l) in array.elems.into_iter().enumerate() {
                    if i >= $s {
                        panic!("incorrect length, bigger than {}, file: {}", $s, path);
                    }

                    let mut neg = false;

                    let exp = match l {
                        ::lazy_static_include::syn::Expr::Lit(exp) => {
                            exp
                        }
                        ::lazy_static_include::syn::Expr::Unary(exp) => {
                            neg = true;

                            match exp.expr.as_ref() {
                                ::lazy_static_include::syn::Expr::Lit(exp) => {
                                    exp.clone()
                                }
                                _ => {
                                    panic!("incorrect element type, index = {}, file: {}", i, path);
                                }
                            }
                        }
                        _ => {
                            panic!("incorrect element type, index = {}, file: {}", i, path);
                        }
                    };

                    match exp.lit {
                        ::lazy_static_include::syn::Lit::Float(f) => {
                            if f.suffix() == ::lazy_static_include::syn::FloatSuffix::F64 {
                                panic!("incorrect element type, index = {}, file: {}", i, path);
                            }

                            let f = if neg {
                                -1.0 * f.value()
                            } else {
                                f.value()
                            };

                            result[i] = f as f32;
                        }
                        ::lazy_static_include::syn::Lit::Int(n) => {
                            let f = n.value() as f32;

                            let ts = n.into_token_stream();

                            let s = ts.into_iter().next().unwrap().to_string();

                            if s.ends_with_caseless_ascii("f32") {
                                let f = if neg {
                                    -1.0 * f
                                } else {
                                    f
                                };

                                result[i] = f as f32;
                            } else {
                                panic!("incorrect element type, index = {}, file: {}", i, path);
                            }
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
    }
}

#[cfg(all(debug_assertions, not(feature = "no_std")))]
#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_array_inner_f64 {
    ( $name:ident: [$t:ident; $s:expr], $path:expr ) => {
        {
            use ::std::fs::File;
            use ::std::io::Read;
            use ::lazy_static_include::syn::export::ToTokens;
            use ::lazy_static_include::starts_ends_with_caseless::EndsWithCaseless;

            let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", $path);

            let v = {
                let mut f = File::open(&path).unwrap();

                let mut v: Vec<u8> = Vec::new();

                f.read_to_end(&mut v).unwrap();

                v
            };

            let s = String::from_utf8(v).unwrap();

            let s = s.trim();

            let mut result = [0 as $t; $s];

            if let Ok(::lazy_static_include::syn::Expr::Array(array)) = ::lazy_static_include::syn::parse_str(s) {
                for (i, l) in array.elems.into_iter().enumerate() {
                    if i >= $s {
                        panic!("incorrect length, bigger than {}, file: {}", $s, path);
                    }

                    let mut neg = false;

                    let exp = match l {
                        ::lazy_static_include::syn::Expr::Lit(exp) => {
                            exp
                        }
                        ::lazy_static_include::syn::Expr::Unary(exp) => {
                            neg = true;

                            match exp.expr.as_ref() {
                                ::lazy_static_include::syn::Expr::Lit(exp) => {
                                    exp.clone()
                                }
                                _ => {
                                    panic!("incorrect element type, index = {}, file: {}", i, path);
                                }
                            }
                        }
                        _ => {
                            panic!("incorrect element type, index = {}, file: {}", i, path);
                        }
                    };

                    match exp.lit {
                        ::lazy_static_include::syn::Lit::Float(f) => {
                            if f.suffix() == ::lazy_static_include::syn::FloatSuffix::F32 {
                                panic!("incorrect element type, index = {}, file: {}", i, path);
                            }

                            let f = if neg {
                                -1.0 * f.value()
                            } else {
                                f.value()
                            };

                            result[i] = f as f64;
                        }
                        ::lazy_static_include::syn::Lit::Int(n) => {
                            let f = n.value() as f64;

                            let ts = n.into_token_stream();

                            let s = ts.into_iter().next().unwrap().to_string();

                            if s.ends_with_caseless_ascii("f64") {
                                let f = if neg {
                                    -1.0 * f
                                } else {
                                    f
                                };

                                result[i] = f as f64;
                            } else {
                                panic!("incorrect element type, index = {}, file: {}", i, path);
                            }
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
    }
}

#[cfg(all(debug_assertions, not(feature = "no_std")))]
#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_array_inner {
    ( $name:ident: [isize; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_i!($name: [isize; $s], $path)
        }
    };
    ( $name:ident: [i8; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_i!($name: [i8; $s], $path)
        }
    };
    ( $name:ident: [i16; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_i!($name: [i16; $s], $path)
        }
    };
    ( $name:ident: [i32; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_i!($name: [i32; $s], $path)
        }
    };
    ( $name:ident: [i64; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_i!($name: [i64; $s], $path)
        }
    };
    ( $name:ident: [i128; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_i128!($name: [i128; $s], $path)
        }
    };
    ( $name:ident: [f32; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_f32!($name: [f32; $s], $path)
        }
    };
    ( $name:ident: [f64; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_f64!($name: [f64; $s], $path)
        }
    };
    ( $name:ident: [usize; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_u!($name: [usize; $s], $path)
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
            lazy_static_include_array_inner_u128!($name: [u128; $s], $path)
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
    ( $name:ident: [&'static str; $s:expr], Vec, $($paths:expr), + ) => {
        {
            let mut v: Vec<[&'static str; $s]> = Vec::with_capacity(lazy_static_include_counter!(Vec $(, $paths)+));

            $(
                v.push(lazy_static_include_array_inner!($name: [&'static str; $s], $paths));
            )+

            v
        }
    };
    ( $name:ident: [&'static str; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_s!($name: [&'static str; $s], $path)
        }
    };
    ( $name:ident: [$t:ident; $s:expr], Vec, $($paths:expr), + ) => {
        {
            let mut v: Vec<[$t; $s]> = Vec::with_capacity(lazy_static_include_counter!(Vec $(, $paths)+));

            $(
                v.push(lazy_static_include_array_inner!($name: [$t; $s], $paths));
            )+

            v
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
    ( $name:ident: [&'static str; $s:expr], $path:expr $(,)* ) => {
        lazy_static! {
            static ref $name: [&'static str; $s] = lazy_static_include_array_inner!($name: [&'static str; $s], $path);
        }
    };
    ( $name:ident: [&'static str; $s:expr], $path:expr, $($paths:expr), + $(,)* ) => {
        lazy_static! {
            static ref $name: Vec<[&'static str; $s]> = lazy_static_include_array_inner!($name: [&'static str; $s], $path $(, $paths)+);
        }
    };
    ( pub $name:ident: [&'static str; $s:expr], $path:expr $(,)* ) => {
        lazy_static! {
            pub static ref $name: [&'static str; $s] = lazy_static_include_array_inner!($name: [&'static str; $s], $path);
        }
    };
    ( pub $name:ident: [&'static str; $s:expr], $path:expr, $($paths:expr), + $(,)* ) => {
        lazy_static! {
            pub static ref $name: Vec<[&'static str; $s]> = lazy_static_include_array_inner!($name: [&'static str; $s], $path $(, $paths)+);
        }
    };
    ( $name:ident: [$t:ident; $s:expr], $path:expr $(,)* ) => {
        lazy_static! {
            static ref $name: [$t; $s] = lazy_static_include_array_inner!($name: [$t; $s], $path);
        }
    };
    ( $name:ident: [$t:ident; $s:expr], $path:expr, $($paths:expr), + $(,)* ) => {
        lazy_static! {
            static ref $name: Vec<[$t; $s]> = lazy_static_include_array_inner!($name: [$t; $s], $path $(, $paths)+);
        }
    };
    ( pub $name:ident: [$t:ident; $s:expr], $path:expr $(,)* ) => {
        lazy_static! {
            pub static ref $name: [$t; $s] = lazy_static_include_array_inner!($name: [$t; $s], $path);
        }
    };
    ( pub $name:ident: [$t:ident; $s:expr], $path:expr, $($paths:expr), + $(,)* ) => {
        lazy_static! {
            pub static ref $name: Vec<[$t; $s]> = lazy_static_include_array_inner!($name: [$t; $s], $path $(, $paths)+);
        }
    };
}

#[macro_export]
macro_rules! lazy_static_include_array_vec {
    ( $name:ident: [&'static str; $s:expr] $(, $paths:expr)+ $(,)* ) => {
        lazy_static! {
            static ref $name: Vec<[&'static str; $s]> = lazy_static_include_array_inner!($name: [&'static str; $s], Vec $(, $paths)+);
        }
    };
    ( pub $name:ident: [&'static str; $s:expr], $($paths:expr), + $(,)* ) => {
        lazy_static! {
            pub static ref $name: Vec<[&'static str; $s]> = lazy_static_include_array_inner!($name: [&'static str; $s], Vec $(, $paths)+);
        }
    };
    ( $name:ident: [$t:ident; $s:expr], $($paths:expr), + $(,)* ) => {
        lazy_static! {
            static ref $name: Vec<[$t; $s]> = lazy_static_include_array_inner!($name: [$t; $s], Vec $(, $paths)+);
        }
    };
    ( pub $name:ident: [$t:ident; $s:expr], $($paths:expr), + $(,)* ) => {
        lazy_static! {
            pub static ref $name: Vec<[$t; $s]> = lazy_static_include_array_inner!($name: [$t; $s], Vec $(, $paths)+);
        }
    };
}