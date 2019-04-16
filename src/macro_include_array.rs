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

            let len = s.len();

            if len < 2 || &s[0..1] != "[" || &s[len - 1..len] != "]" {
                panic!("incorrect array, file: {}", path);
            }

            let mut result = [false; $s];
            let mut p = 0usize;

            let mut concating = false;
            let mut tmp = String::new();

            let len = s.chars().count();

            for c in s.chars().into_iter().skip(1).take(len - 2) {
                 if c == ' ' || c == '\n'  || c == '\t'{
                    if concating {
                        if c == '\n' {
                            panic!("incorrect array, file: {}", path);
                        } else {
                            tmp.push(c);
                        }
                    }else {
                        continue;
                    }
                } else if c == ',' {
                    if concating {
                        panic!("incorrect array, file: {}", path);
                    }
                } else if c == 't' || c == 'f' {
                    if concating {
                        panic!("incorrect array, file: {}", path);
                    } else {
                        concating = true;

                        tmp.push(c);
                    }
                } else if c == 'e' {
                    if concating {
                        if "tru".eq(tmp.as_str()) {
                            result[p] = true;
                        } else if "fals".eq(tmp.as_str()) {
                            result[p] = false;
                        } else {
                            panic!("incorrect array, file: {}", path);
                        }

                        p += 1;

                        tmp.clear();

                        concating = false;
                    } else {
                        panic!("incorrect array, file: {}", path);
                    }
                } else {
                    if concating {
                        tmp.push(c);
                    } else {
                        panic!("incorrect array, file: {}", path);
                    }
                }
            }

            if concating {
                panic!("incorrect array, file: {}", path);
            }

            if p != $s {
                panic!("incorrect length, {} != {}, file: {}", p, $s, path);
            }

            result
        }
    }
}

#[cfg(debug_assertions)]
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

            let len = s.len();

            if len < 2 || &s[0..1] != "[" || &s[len - 1..len] != "]" {
                panic!("incorrect array, file: {}", path);
            }

            let mut result = ['\0'; $s];
            let mut p = 0usize;

            let mut concating = false;
            let mut cannot_concating = false;
            let mut tmp = '\0';

            let len = s.chars().count();

            for c in s.chars().into_iter().skip(1).take(len - 2) {
                 if c == ' ' || c == '\n'  || c == '\t'{
                    if concating {
                        if c == '\n' {
                            panic!("incorrect array, file: {}", path);
                        } else {
                            tmp = c;
                        }
                    }else {
                        continue;
                    }
                } else if c == ',' {
                    if concating {
                        if cannot_concating {
                            panic!("incorrect array, file: {}", path);
                        }
                        tmp = c;
                        cannot_concating = true;
                    } else {
                        cannot_concating = false;
                    }
                } else if c == '\'' {
                    if concating {
                        result[p] = tmp;
                        p += 1;

                        concating = false;
                    } else {
                        if cannot_concating {
                            panic!("incorrect array, file: {}", path);
                        }
                        concating = true;
                    }
                } else {
                    if concating {
                        if cannot_concating {
                            panic!("incorrect array, file: {}", path);
                        }
                        tmp = c;
                        cannot_concating = true;
                    } else {
                        panic!("incorrect array, file: {}", path);
                    }
                }
            }

            if concating {
                panic!("incorrect array, file: {}", path);
            }

            if p != $s {
                panic!("incorrect length, {} != {}, file: {}", p, $s, path);
            }

            result
        }
    }
}

#[cfg(debug_assertions)]
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

            let len = s.len();

            if len < 2 || &s[0..1] != "[" || &s[len - 1..len] != "]" {
                panic!("incorrect array, file: {}", path);
            }

            let mut result = Vec::with_capacity($s);

            let mut concating = false;
            let mut cannot_concating = false;
            let mut tmp = String::new();

            let len = s.chars().count();

            for c in s.chars().into_iter().skip(1).take(len - 2) {
                 if c == ' ' || c == '\n'  || c == '\t'{
                    if concating {
                        if c == '\n' {
                            panic!("incorrect array, file: {}", path);
                        } else {
                            tmp.push(c);
                        }
                    }else {
                        continue;
                    }
                } else if c == ',' {
                    if concating {
                        result.push(tmp.clone());
                    } else {
                        cannot_concating = false;
                    }
                } else if c == '"' {
                    if concating {
                        result.push(tmp.clone());

                        concating = false;

                        tmp.clear();

                        cannot_concating = true;
                    } else {
                        if cannot_concating {
                            panic!("incorrect array, file: {}", path);
                        }
                        concating = true;
                    }
                } else {
                    if concating {
                        tmp.push(c);
                    } else {
                        panic!("incorrect array, file: {}", path);
                    }
                }
            }

            if concating {
                panic!("incorrect array, file: {}", path);
            }

            if result.len() != $s {
                panic!("incorrect length, {} != {}, file: {}", result.len(), $s, path);
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
        }
    }
}

#[cfg(debug_assertions)]
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

            let len = s.len();

            if len < 2 || &s[0..1] != "[" || &s[len - 1..len] != "]" {
                panic!("incorrect array, file: {}", path);
            }

            let mut result = [0 as $t; $s];

            let mut p = 0usize;

            let mut concating = false;
            let mut concating_ignore = false;
            let mut concating_ignore_s = String::new();
            let mut tmp = 0 as $t;

            let len = s.chars().count();

            for c in s.chars().into_iter().skip(1).take(len - 2) {
                 if c == ' ' || c == '\n' || c == '\t'{
                    if (c == '\n'  || c == '\t') && concating {
                        panic!("incorrect array, file: {}", path);
                    }
                    continue;
                } else if c == '_' {
                    if concating {
                        continue;
                    } else {
                        panic!("incorrect array, file: {}", path);
                    }
                } else if c == ',' {
                    if concating {
                        if concating_ignore {
                            if stringify!($t).ne(concating_ignore_s.as_str()) {
                                panic!("incorrect array, file: {}", path);
                            }
                        }

                        result[p] = tmp;
                        p += 1;

                        concating = false;
                        concating_ignore = false;
                        concating_ignore_s.clear();

                        tmp = 0 as $t;
                    } else {
                        panic!("incorrect array, file: {}", path);
                    }
                } else if c == '-' {
                    panic!("incorrect array, file: {}", path);
                } else if concating_ignore {
                    concating_ignore_s.push(c);
                } else if c >= '0' && c <= '9'{
                    let mut n = ((c as u32) - ('0' as u32)) as $t;

                    tmp = tmp * (10 as $t) + n;

                    concating = true;
                } else if c == '.' {
                    panic!("incorrect array, file: {}", path);
                } else {
                    if concating {
                        concating_ignore = true;
                        concating_ignore_s.push(c);
                    } else {
                        panic!("incorrect array, file: {}", path);
                    }
                }
            }

            if concating {
                if concating_ignore {
                    if stringify!($t).ne(concating_ignore_s.as_str()) {
                        panic!("incorrect array, file: {}", path);
                    }
                }

                result[p] = tmp;
                p += 1;
            }

            if p != $s {
                panic!("incorrect length, {} != {}, file: {}", p, $s, path);
            }

            result
        }
    }
}

#[cfg(debug_assertions)]
#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_array_inner_if {
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

            let len = s.len();

            if len < 2 || &s[0..1] != "[" || &s[len - 1..len] != "]" {
                panic!("incorrect array, file: {}", path);
            }

            let mut result = [0 as $t; $s];

            let mut p = 0usize;

            let mut concating = false;
            let mut negative = false;
            let mut concating_ignore = false;
            let mut concating_ignore_s = String::new();
            let mut floating = 0usize;
            let mut tmp = 0 as $t;

            let len = s.chars().count();

            for c in s.chars().into_iter().skip(1).take(len - 2) {
                 if c == ' ' || c == '\n'  || c == '\t'{
                    if (c == '\n'  || c == '\t') && concating {
                        panic!("incorrect array, file: {}", path);
                    }
                    continue;
                } else if c == '_' {
                    if concating {
                        continue;
                    } else {
                        panic!("incorrect array, file: {}", path);
                    }
                } else if c == ',' {
                    if concating {
                        if concating_ignore {
                            if stringify!($t).ne(concating_ignore_s.as_str()) {
                                panic!("incorrect array, file: {}", path);
                            }
                        } else {
                            if floating == 0 && stringify!($t).starts_with("f") {
                                panic!("incorrect array, file: {}", path);
                            }
                        }

                        if negative {
                            result[p] = -tmp;
                        } else {
                            result[p] = tmp;
                        }
                        p += 1;

                        concating = false;
                        negative = false;
                        concating_ignore = false;
                        concating_ignore_s.clear();

                        floating = 0;

                        tmp = 0 as $t;
                    } else {
                        panic!("incorrect array, file: {}", path);
                    }
                } else if c == '-' {
                    if concating {
                        panic!("incorrect array, file: {}", path);
                    } else {
                        negative = true;
                    }
                } else if concating_ignore {
                    concating_ignore_s.push(c);
                } else if c >= '0' && c <= '9'{
                    let mut n = ((c as u32) - ('0' as u32)) as $t;

                    if floating > 0 {
                        for _ in 0..floating {
                            n /= 10 as $t;
                        }

                        floating += 1;

                        tmp = tmp + n;
                    } else {
                        tmp = tmp * (10 as $t) + n;
                    }

                    concating = true;
                } else if c == '.' {
                    floating = 1;
                } else {
                    if concating {
                        concating_ignore = true;
                        concating_ignore_s.push(c);
                    } else {
                        panic!("incorrect array, file: {}", path);
                    }
                }
            }

            if concating {
                if concating_ignore {
                    if stringify!($t).ne(concating_ignore_s.as_str()) {
                        panic!("incorrect array, file: {}", path);
                    }
                } else {
                    if floating == 0 && stringify!($t).starts_with("f") {
                        panic!("incorrect array, file: {}", path);
                    }
                }

                if negative {
                    result[p] = -tmp;
                } else {
                    result[p] = tmp;
                }
                p += 1;
            } else {
                if negative {
                    panic!("incorrect array, file: {}", path);
                }
            }

            if p != $s {
                panic!("incorrect length, {} != {}, file: {}", p, $s, path);
            }

            result
        }
    }
}

#[cfg(debug_assertions)]
#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_array_inner {
    ( $name:ident: [i8; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_if!($name: [i8; $s], $path)
        }
    };
    ( $name:ident: [i16; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_if!($name: [i16; $s], $path)
        }
    };
    ( $name:ident: [i32; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_if!($name: [i32; $s], $path)
        }
    };
    ( $name:ident: [i64; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_if!($name: [i64; $s], $path)
        }
    };
    ( $name:ident: [i128; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_if!($name: [i128; $s], $path)
        }
    };
    ( $name:ident: [f32; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_if!($name: [f32; $s], $path)
        }
    };
    ( $name:ident: [f64; $s:expr], $path:expr ) => {
        {
            lazy_static_include_array_inner_if!($name: [f64; $s], $path)
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
#[deprecated(since = "1.3.0", note = "extremely unstable, it should be implemented by the `syn` crate")]
macro_rules! lazy_static_include_array {
    ( $name:ident: [&'static str; $s:expr], $path:expr $(,)* ) => {
        lazy_static! {
            #[deprecated(since = "1.3.0", note = "extremely unstable, it should be implemented by the `syn` crate")]
            static ref $name: [&'static str; $s] = lazy_static_include_array_inner!($name: [&'static str; $s], $path);
        }
    };
    ( $name:ident: [&'static str; $s:expr], $path:expr, $($paths:expr), + $(,)* ) => {
        lazy_static! {
            #[deprecated(since = "1.3.0", note = "extremely unstable, it should be implemented by the `syn` crate")]
            static ref $name: Vec<[&'static str; $s]> = lazy_static_include_array_inner!($name: [&'static str; $s], $path $(, $paths)+);
        }
    };
    ( pub $name:ident: [&'static str; $s:expr], $path:expr $(,)* ) => {
        lazy_static! {
            #[deprecated(since = "1.3.0", note = "extremely unstable, it should be implemented by the `syn` crate")]
            pub static ref $name: [&'static str; $s] = lazy_static_include_array_inner!($name: [&'static str; $s], $path);
        }
    };
    ( pub $name:ident: [&'static str; $s:expr], $path:expr, $($paths:expr), + $(,)* ) => {
        lazy_static! {
            #[deprecated(since = "1.3.0", note = "extremely unstable, it should be implemented by the `syn` crate")]
            pub static ref $name: Vec<[&'static str; $s]> = lazy_static_include_array_inner!($name: [&'static str; $s], $path $(, $paths)+);
        }
    };
    ( $name:ident: [$t:ident; $s:expr], $path:expr $(,)* ) => {
        lazy_static! {
            #[deprecated(since = "1.3.0", note = "extremely unstable, it should be implemented by the `syn` crate")]
            static ref $name: [$t; $s] = lazy_static_include_array_inner!($name: [$t; $s], $path);
        }
    };
    ( $name:ident: [$t:ident; $s:expr], $path:expr, $($paths:expr), + $(,)* ) => {
        lazy_static! {
            #[deprecated(since = "1.3.0", note = "extremely unstable, it should be implemented by the `syn` crate")]
            static ref $name: Vec<[$t; $s]> = lazy_static_include_array_inner!($name: [$t; $s], $path $(, $paths)+);
        }
    };
    ( pub $name:ident: [$t:ident; $s:expr], $path:expr $(,)* ) => {
        lazy_static! {
            #[deprecated(since = "1.3.0", note = "extremely unstable, it should be implemented by the `syn` crate")]
            pub static ref $name: [$t; $s] = lazy_static_include_array_inner!($name: [$t; $s], $path);
        }
    };
    ( pub $name:ident: [$t:ident; $s:expr], $path:expr, $($paths:expr), + $(,)* ) => {
        lazy_static! {
            #[deprecated(since = "1.3.0", note = "extremely unstable, it should be implemented by the `syn` crate")]
            pub static ref $name: Vec<[$t; $s]> = lazy_static_include_array_inner!($name: [$t; $s], $path $(, $paths)+);
        }
    };
}

#[macro_export]
macro_rules! lazy_static_include_array_vec {
    ( $name:ident: [&'static str; $s:expr] $(, $paths:expr)+ $(,)* ) => {
        lazy_static! {
            #[deprecated(since = "1.3.0", note = "extremely unstable, it should be implemented by the `syn` crate")]
            static ref $name: Vec<[&'static str; $s]> = lazy_static_include_array_inner!($name: [&'static str; $s], Vec $(, $paths)+);
        }
    };
    ( pub $name:ident: [&'static str; $s:expr], $($paths:expr), + $(,)* ) => {
        lazy_static! {
            #[deprecated(since = "1.3.0", note = "extremely unstable, it should be implemented by the `syn` crate")]
            pub static ref $name: Vec<[&'static str; $s]> = lazy_static_include_array_inner!($name: [&'static str; $s], Vec $(, $paths)+);
        }
    };
    ( $name:ident: [$t:ident; $s:expr], $($paths:expr), + $(,)* ) => {
        lazy_static! {
            #[deprecated(since = "1.3.0", note = "extremely unstable, it should be implemented by the `syn` crate")]
            static ref $name: Vec<[$t; $s]> = lazy_static_include_array_inner!($name: [$t; $s], Vec $(, $paths)+);
        }
    };
    ( pub $name:ident: [$t:ident; $s:expr], $($paths:expr), + $(,)* ) => {
        lazy_static! {
            #[deprecated(since = "1.3.0", note = "extremely unstable, it should be implemented by the `syn` crate")]
            pub static ref $name: Vec<[$t; $s]> = lazy_static_include_array_inner!($name: [$t; $s], Vec $(, $paths)+);
        }
    };
}