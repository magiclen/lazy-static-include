#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static_include_counter {
    () => (0usize);
    ( Vec $(, $xs:expr)* $(,)* ) => ($crate::lazy_static_include_counter!($($xs, )*));
    ( $x:expr $(, $xs:expr)* $(,)* ) => (1usize + $crate::lazy_static_include_counter!($($xs, )*));
}
