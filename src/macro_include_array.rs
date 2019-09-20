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

#[cfg(debug_assertions)]
#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_array_inner_b {
    ($name:ident : [$t:ident; $s:expr], $path:expr) => {{
        use std::fs::File;
        use std::io::Read;
        use std::mem;

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
    }};
}

#[cfg(debug_assertions)]
#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_array_inner_c {
    ($name:ident : [$t:ident; $s:expr], $path:expr) => {{
        use std::fs::File;
        use std::io::Read;
        use std::mem;

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
    }};
}

#[cfg(debug_assertions)]
#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_array_inner_s {
    ($name:ident : [&'static str; $s:expr], $path:expr) => {{
        use std::fs::File;
        use std::io::Read;
        use std::mem;

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
    }};
}

#[cfg(debug_assertions)]
#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_array_inner_u {
    ($name:ident : [$t:ident; $s:expr], $path:expr) => {{
        use std::fs::File;
        use std::io::Read;

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
    }};
}

#[cfg(debug_assertions)]
#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_array_inner_i {
    ($name:ident : [$t:ident; $s:expr], $path:expr) => {{
        use std::fs::File;
        use std::io::Read;

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
    }};
}

#[cfg(debug_assertions)]
#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_array_inner_f {
    ($name:ident : [$t:ident; $s:expr], $path:expr) => {{
        use std::fs::File;
        use std::io::Read;
        use $crate::starts_ends_with_caseless::EndsWithCaseless;
        use $crate::syn::export::ToTokens;

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
    }};
}

#[cfg(debug_assertions)]
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
            lazy_static_include_array_inner_i!($name: [i128; $s], $path)
        }
    };
    ( $name:ident: [f32; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_f!($name: [f32; $s], $path)
        }
    };
    ( $name:ident: [f64; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_f!($name: [f64; $s], $path)
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
            lazy_static_include_array_inner_u!($name: [u128; $s], $path)
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
    ( pub($($vis:tt)*) $name:ident: [&'static str; $s:expr], $path:expr $(,)* ) => {
        lazy_static! {
            pub($($vis)*) static ref $name: [&'static str; $s] = lazy_static_include_array_inner!($name: [&'static str; $s], $path);
        }
    };
    ( pub $name:ident: [&'static str; $s:expr], $path:expr, $($paths:expr), + $(,)* ) => {
        lazy_static! {
            pub static ref $name: Vec<[&'static str; $s]> = lazy_static_include_array_inner!($name: [&'static str; $s], $path $(, $paths)+);
        }
    };
    ( pub($($vis:tt)*) $name:ident: [&'static str; $s:expr], $path:expr, $($paths:expr), + $(,)* ) => {
        lazy_static! {
            pub($($vis)*) static ref $name: Vec<[&'static str; $s]> = lazy_static_include_array_inner!($name: [&'static str; $s], $path $(, $paths)+);
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
    ( pub($($vis:tt)*) $name:ident: [$t:ident; $s:expr], $path:expr $(,)* ) => {
        lazy_static! {
            pub($($vis)*) static ref $name: [$t; $s] = lazy_static_include_array_inner!($name: [$t; $s], $path);
        }
    };
    ( pub $name:ident: [$t:ident; $s:expr], $path:expr, $($paths:expr), + $(,)* ) => {
        lazy_static! {
            pub static ref $name: Vec<[$t; $s]> = lazy_static_include_array_inner!($name: [$t; $s], $path $(, $paths)+);
        }
    };
    ( pub($($vis:tt)*) $name:ident: [$t:ident; $s:expr], $path:expr, $($paths:expr), + $(,)* ) => {
        lazy_static! {
            pub($($vis)*) static ref $name: Vec<[$t; $s]> = lazy_static_include_array_inner!($name: [$t; $s], $path $(, $paths)+);
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
    ( pub($($vis:tt)*) $name:ident: [&'static str; $s:expr], $($paths:expr), + $(,)* ) => {
        lazy_static! {
            pub($($vis)*) static ref $name: Vec<[&'static str; $s]> = lazy_static_include_array_inner!($name: [&'static str; $s], Vec $(, $paths)+);
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
    ( pub($($vis:tt)*) $name:ident: [$t:ident; $s:expr], $($paths:expr), + $(,)* ) => {
        lazy_static! {
            pub($($vis)*) static ref $name: Vec<[$t; $s]> = lazy_static_include_array_inner!($name: [$t; $s], Vec $(, $paths)+);
        }
    };
}
