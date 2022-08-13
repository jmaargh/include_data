#![no_std]
#![allow(clippy::modulo_one)]

//! Prior art: <https://jack.wrenn.fyi/blog/include-transmute/>
//! - transmute version does not work for slices because it requires a reference
//!   to a signed type
//!
//! For slices, requires beta or nightly until [this issue](https://github.com/rust-lang/rust/issues/67456)
//! is stabilised.
//!
//! Is safe because will only construct aligned and properly sized slices for
//! types that are valid for all bit patterns.

/// Simple re-export of `include_bytes`
pub use ::core::include_bytes as include_u8;

/// Includes a file as a static reference to a slice of any primitive integers.
///
/// For any primitive integer type `T`, `include_ints(T, path)` will return
/// a `&'static [T]` slice containing the contents of the file at `path`. This
/// is guaranteed to be properly aligned.
///
/// Will throw a compiler error if the included file will not fit into a
/// `&[T]` slice. That is, if the file size is not divisible by
/// `size_of::<T>()`.
///
/// Uses [`std::include_bytes`](std::include_bytes) and therefore has the same
/// portability limitations on the path.
///
/// [incb]: std::include_bytes
#[macro_export]
macro_rules! include_ints {
    ($int_ty:ty, $path:literal $(,)?) => {{
        const INT_SIZE: usize = ::core::mem::size_of::<$int_ty>();

        static ALIGNED: &$crate::AlignedAs<$int_ty, [u8]> = &$crate::AlignedAs {
            _align: [],
            bytes: *include_bytes!($path),
        };

        let byte_slice: &[u8] = &ALIGNED.bytes;

        let _requires_int: $int_ty = 0;
        assert!(
            byte_slice.len() % INT_SIZE == 0,
            "Included file size is not divisible by int size",
        );

        let out_slice: &'static [$int_ty] = unsafe {
            ::core::slice::from_raw_parts(byte_slice.as_ptr().cast(), byte_slice.len() / INT_SIZE)
        };

        out_slice
    }};
}

/// Alias of [`include_ints(u16, path)`](include_ints). Returns a `&'static [u16]`.
#[macro_export]
macro_rules! include_u16 {
    ($path:literal $(,)?) => {
        include_ints!(u16, $path)
    };
}

/// Alias of [`include_ints(u32, path)`](include_ints). Returns a `&'static [u32]`.
#[macro_export]
macro_rules! include_u32 {
    ($path:literal $(,)?) => {
        include_ints!(u32, $path)
    };
}

/// Alias of [`include_ints(u64, path)`](include_ints). Returns a `&'static [u64]`.
#[macro_export]
macro_rules! include_u64 {
    ($path:literal $(,)?) => {
        include_ints!(u64, $path)
    };
}

/// Alias of [`include_ints(u128, path)`](include_ints). Returns a `&'static [u128]`.
#[macro_export]
macro_rules! include_u128 {
    ($path:literal $(,)?) => {
        include_ints!(u128, $path)
    };
}

/// Alias of [`include_ints(i8, path)`](include_ints). Returns a `&'static [i8]`.
#[macro_export]
macro_rules! include_i8 {
    ($path:literal $(,)?) => {
        include_ints!(i8, $path)
    };
}

/// Alias of [`include_ints(i16, path)`](include_ints). Returns a `&'static [i16]`.
#[macro_export]
macro_rules! include_i16 {
    ($path:literal $(,)?) => {
        include_ints!(i16, $path)
    };
}

/// Alias of [`include_ints(i32, path)`](include_ints). Returns a `&'static [i32]`.
#[macro_export]
macro_rules! include_i32 {
    ($path:literal $(,)?) => {
        include_ints!(i32, $path)
    };
}

/// Alias of [`include_ints(i64, path)`](include_ints). Returns a `&'static [i64]`.
#[macro_export]
macro_rules! include_i64 {
    ($path:literal $(,)?) => {
        include_ints!(i64, $path)
    };
}

/// Alias of [`include_ints(i128, path)`](include_ints). Returns a `&'static [i128]`.
#[macro_export]
macro_rules! include_i128 {
    ($path:literal $(,)?) => {
        include_ints!(i128, $path)
    };
}

/// Includes a file as a static reference to a slice of any primitive floating
/// point values (`f32` or `f64`).
///
/// For any primitive floating point type `T`, `include_floats(T, path)` will
/// return a `&'static [T]` slice containing the contents of the file at `path`.
/// This is guaranteed to be properly aligned.
///
/// Will throw a compiler error if the included file will not fit into a
/// `&[T]` slice. That is, if the file size is not divisible by
/// `size_of::<T>()`.
///
/// Uses `std::include_bytes` and therefore has the same portability limitations
/// on the path.
#[macro_export]
macro_rules! include_floats {
    ($float_ty:ty, $path:literal $(,)?) => {{
        const FLOAT_SIZE: usize = ::core::mem::size_of::<$float_ty>();

        static ALIGNED: &$crate::AlignedAs<$float_ty, [u8]> = &$crate::AlignedAs {
            _align: [],
            bytes: *include_bytes!($path),
        };

        let byte_slice: &[u8] = &ALIGNED.bytes;

        let _requires_float: $float_ty = 0.0;
        assert!(
            byte_slice.len() % FLOAT_SIZE == 0,
            "Included file size is not divisible by int size",
        );

        let out_slice: &'static [$float_ty] = unsafe {
            ::core::slice::from_raw_parts(byte_slice.as_ptr().cast(), byte_slice.len() / FLOAT_SIZE)
        };

        out_slice
    }};
}

/// Alias of [`include_floats(f32, path)`](include_floats)
#[macro_export]
macro_rules! include_f32 {
    ($path:literal $(,)?) => {
        include_floats!(f32, $path)
    };
}

/// Alias of [`include_floats(f64, path)`](include_floats)
#[macro_export]
macro_rules! include_f64 {
    ($path:literal $(,)?) => {
        include_foats!(f64, $path)
    };
}

/// Force alignment of the `bytes` member to match that of type T.
/// `Bytes` is simply `[u8]` but handles that it is unsized.
#[doc(hidden)]
#[repr(C)]
pub struct AlignedAs<T, Bytes: ?Sized> {
    pub _align: [T; 0],
    pub bytes: Bytes,
}
