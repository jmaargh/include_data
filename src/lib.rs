#![no_std]
#![allow(clippy::modulo_one)]

//! Prior art: https://jack.wrenn.fyi/blog/include-transmute/
//! - transmute version does not work for slices because it requires a reference
//!   to a signed type
//!
//! For slices, requires beta or nightly until [this issue](https://github.com/rust-lang/rust/issues/67456)
//! is stabilised.

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
/// Uses `std::include_bytes` and therefore has the same portability limitations
/// on the path.
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

/// Force alignment of the `bytes` member to match that of type T.
/// `Bytes` is simply `[u8]` but handles that it is unsized.
#[repr(C)]
pub struct AlignedAs<T, Bytes: ?Sized> {
    pub _align: [T; 0],
    pub bytes: Bytes,
}
