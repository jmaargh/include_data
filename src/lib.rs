#![no_std]
#![allow(clippy::modulo_one)]

//! # include_data - Include typed data directly in your executable
//!
//! The primary API is provided by two macros:
//! - [`include_data`] - include static data as any plain-old-data type
//! - [`include_slice`] - include static data as `&'static [T]` slice for any
//!                       plain-old-data `T`
//!
//! Soundness of types for this purpose is guaranteed via the
//! [`Pod`][bytemuck::Pod] trait from the `bytemuck` crate. This trait can be
//! implemented on any type and so long as that implementation is sound, then
//! these macros are also sound.
//!
//! So for core library types, the following works out of the box:
//! ```
//! # use include_data::{include_data, include_slice};
//! static MY_INTEGER: i32 = include_data!("../tests/test_data/file_exactly_4_bytes_long");
//! static SOME_TEXT: &[u32] = include_slice!(u32, "../tests/test_data/some_utf-32_file");
//! const FOUR_BYTES: [u8; 4] = include_data!("../tests/test_data/file_exactly_4_bytes_long");
//! ```
//! Note that `include_data` works with `const`, while `include_slice` only
//! supports `static`.
//!
//! For custom types:
//! ```
//! # use include_data::include_data;
//! #[repr(C)]
//! #[derive(Copy, Clone)]
//! struct Foo {
//!     integer: u16,
//!     pair: [u8; 2],
//! }
//!
//! // Safety: the type `Foo` has been checked to satisfy all requirements of
//! // `Pod`.
//! unsafe impl bytemuck::Zeroable for Foo {}
//! unsafe impl bytemuck::Pod for Foo {}
//!
//! static FOO_DATA: Foo = include_data!("../tests/test_data/file_exactly_4_bytes_long");
//! ```
//!
//! If necessary, this crate also provides the [`include_unsafe`] macro,
//! which is sound if and only if the included file is a valid bit pattern
//! for the target type, but this is not checked. This should be avoided unless
//! absolutely necessary, since it is very unsafe and its soundness is fragile
//! (in some cases, soundness may be broken by a compiler update). See the macro
//! docs for more details.
//! ```
//! # use include_data::include_unsafe;
//! #[repr(C)]
//! struct StructWithPadding {
//!     byte: u8,
//!     two_bytes: u16,
//! }
//!
//! // Safety: we guarantee that the included file contains bytes which are
//! // a valid bit-pattern for our struct, when compiled on this host.
//! static BAR_DATA: StructWithPadding = unsafe { include_unsafe!("../tests/test_data/file_exactly_4_bytes_long") };
//! ```
//!
//! ## Platform-specific behaviour
//!
//! The interpretation of multi-byte sequences depends on a machine's
//! endianness. In the case of these macros, multi-byte sequences will be
//! interpreted into types according to the endianness of the compilation
//! target, not the compilation host machine.
//!
//! The interpreation of paths passed to these macros is host-platform specific
//! and identical to that of [`core::include_bytes`].

#[doc(hidden)]
pub use bytemuck;

/// Include data from a file as static data in the executable, of a type that
/// implements [`bytemuck::Pod`].
///
/// Can assign to both `static` and `const` variables.
///
/// A compiler error will be thrown if the source file is not the same size as
/// the target type. The path is interpreted by [`core::include_bytes`] and is
/// host-platform-specific. The interpretation of multi-byte sequences (e.g.,
/// a `u32` which occupies 4 bytes) is according to the endianness of the target
/// platform.
///
/// # Example
/// ```
/// # use include_data::include_data;
/// const MY_INTEGER: i32 = include_data!("../tests/test_data/file_exactly_4_bytes_long");
/// static FOUR_BYTES: [u8; 4] = include_data!("../tests/test_data/file_exactly_4_bytes_long");
/// ```
///
/// # Safety
///
/// This macro is safe. However, if used on a custom type, that type must
/// implement [`bytemuck::Pod`]. Implementing that trait has very struct safety
/// requirements which must be observed.
#[macro_export]
macro_rules! include_data {
    ($file:expr) => {{
        const fn typecheck<T: $crate::bytemuck::Pod>(src: T) -> T {
            src
        }

        // Safety: transmuting into a `Pod` type is always safe (as all bit
        // patterns are safe). Alignment of the output type is guaranteed by
        // `transmute`.
        typecheck(unsafe { ::core::mem::transmute(*::core::include_bytes!($file)) })
    }};
}

/// Include data from a file as static data in the executable, without checking
/// validity.
///
/// **Warning:** This macro is **very** unsafe. If at all possible, other macros
/// in this crate should be preferred: even if that makes a runtime conversion
/// necessary, that is often a good tradeoff rather than maintaining the
/// soundness of using this macro. See below for full safety requirements.
///
/// Can assign to both `static` and `const` variables.
///
/// A compiler error will be thrown if the source file is not the same size as
/// the target type. The path is interpreted by [`core::include_bytes`] and is
/// host-platform-specific. The interpretation of multi-byte sequences (e.g.,
/// a `u32` which occupies 4 bytes) is according to the endianness of the target
/// platform.
///
/// # Example
///
/// ```
/// # use include_data::include_unsafe;
/// #[repr(C)]
/// struct StructWithPadding {
///     byte: u8,
///     two_bytes: u16,
/// }
///
/// // Safety: we guarantee that the included file contains bytes which are
/// // a valid bit-pattern for our struct, when compiled on this host.
/// static BAR_DATA: StructWithPadding = unsafe { include_unsafe!("../tests/test_data/file_exactly_4_bytes_long") };
/// ```
///
/// # Safety
///
/// If at all possible, consider using another macro from this crate, even if
/// doing so means performing runtime conversions.
///
/// For a use of this macro to be sound, the bytes of the source file must form
/// a valid bit-pattern for the target type. This macro takes the bytes of the
/// source file in order and then bitwise converts to the target type. It does
/// not handle any endianness issues.
///
/// In particular, note that Rust does not have a stable ABI. This means that
/// the compiler is free to lay out non-primitive types however it pleases:
/// fields will not be in any guaranteed order, there may (or may not) be
/// padding between fields, etc. The layout of a type may change between
/// versions of the compiler and between compiler profiles (for example, debug
/// and release builds could result in different layouts). It is therefore
/// strongly recommended that this macro is only used when
/// [the `repr` attribute](https://doc.rust-lang.org/nomicon/other-reprs.html)
/// is used to force a guaranteed layout.
///
/// Maintaining soundness when using this macro is delicate. In particular,
/// changing the contents of the source file or the definition of the target
/// type at all will often silently result in undefined behaviour.
#[macro_export]
macro_rules! include_unsafe {
    ($file:expr) => {{
        const fn typecheck<T>(src: T) -> T {
            src
        }

        typecheck(::core::mem::transmute(*::core::include_bytes!($file)))
    }};
}

/// Include data from a file as static data, consisting of a slice of
/// [`bytemuck::Pod`] types.
///
/// For any type `T: bytemuck::Pod`, `include_slice!(T, path)` will return
/// a `&'static [T]` slice containing the contents of the file at `path`.
///
/// A compiler error will be thrown source file cannot fit evenly into a `&[T]`
/// slice. That is, if the file size is not divisible by
/// [`size_of::<T>()`][core::mem::size_of]. The path is interpreted by
/// [`core::include_bytes`] and is host-platform-specific. The interpretation of
/// multi-byte sequences (e.g., a `u32` which occupies 4 bytes) is according to
/// the endianness of the target platform.
///
/// While `include_slice!(u8, path)` is supported, [`core::include_bytes`]
/// should be preferred in almost every case as it is a compiler built-in.
///
/// # Why do I have to specify the type twice?
///
/// In order to ensure alignment, these macros internally create a static value
/// with the same alignment as the target type, which the compiler copies the
/// data into at compile-time. Since this is a static value, the type used for
/// alignment must be explicitly specified and cannot be inferred.
///
/// # Example
///
/// ```rust
/// # use include_data::include_slice;
/// static DATA_U32: &[u32] = include_slice!(u32, "../tests/test_data/binary_32");
/// ```
///
/// # Safety
///
/// This macro is safe. However, if used on a custom type, that type must
/// implement [`bytemuck::Pod`]. Implementing that trait has very struct safety
/// requirements which must be observed.
#[macro_export]
macro_rules! include_slice {
    ($target_ty:ty, $file:expr $(,)?) => {{
        const SIZE: usize = ::core::mem::size_of::<$target_ty>();

        static ALIGNED: &$crate::AlignedAs<$target_ty, [u8]> = &$crate::AlignedAs {
            _align: [],
            bytes: *::core::include_bytes!($file),
        };

        let byte_slice: &[u8] = &ALIGNED.bytes;

        assert!(
            byte_slice.len() % SIZE == 0,
            "Included file size is not divisible by target type size",
        );

        let out_slice: &'static [$target_ty] = unsafe {
            ::core::slice::from_raw_parts(byte_slice.as_ptr().cast(), byte_slice.len() / SIZE)
        };

        out_slice
    }};
}

/// Alias of [`include_slice(u8, path)`](include_slice). Returns a `&'static [u8]`.
///
/// Included only for completeness, for almost every case [`core::include_bytes`]
/// should be prefered as it is a compiler built-in.
#[macro_export]
macro_rules! include_u8s {
    ($file:expr $(,)?) => {
        $crate::include_slice!(u8, $file)
    };
}

/// Alias of [`include_slice(u16, path)`](include_slice). Returns a `&'static [u16]`.
#[macro_export]
macro_rules! include_u16s {
    ($file:expr $(,)?) => {
        $crate::include_slice!(u16, $file)
    };
}

/// Alias of [`include_slice(u32, path)`](include_slice). Returns a `&'static [u32]`.
#[macro_export]
macro_rules! include_u32s {
    ($file:expr $(,)?) => {
        $crate::include_slice!(u32, $file)
    };
}

/// Alias of [`include_slice(u64, path)`](include_slice). Returns a `&'static [u64]`.
#[macro_export]
macro_rules! include_u64s {
    ($file:expr $(,)?) => {
        $crate::include_slice!(u64, $file)
    };
}

/// Alias of [`include_slice(u128, path)`](include_slice). Returns a `&'static [u128]`.
#[macro_export]
macro_rules! include_u128s {
    ($file:expr $(,)?) => {
        $crate::include_slice!(u128, $file)
    };
}

/// Alias of [`include_slice(usize, path)`](include_slice). Returns a `&'static [usize]`.
#[macro_export]
macro_rules! include_usizes {
    ($file:expr $(,)?) => {
        $crate::include_slice!(usize, $file)
    };
}

/// Alias of [`include_slice(i8, path)`](include_slice). Returns a `&'static [i8]`.
#[macro_export]
macro_rules! include_i8s {
    ($file:expr $(,)?) => {
        $crate::include_slice!(i8, $file)
    };
}

/// Alias of [`include_slice(i16, path)`](include_slice). Returns a `&'static [i16]`.
#[macro_export]
macro_rules! include_i16s {
    ($file:expr $(,)?) => {
        $crate::include_slice!(i16, $file)
    };
}

/// Alias of [`include_slice(i32, path)`](include_slice). Returns a `&'static [i32]`.
#[macro_export]
macro_rules! include_i32s {
    ($file:expr $(,)?) => {
        $crate::include_slice!(i32, $file)
    };
}

/// Alias of [`include_slice(i64, path)`](include_slice). Returns a `&'static [i64]`.
#[macro_export]
macro_rules! include_i64s {
    ($file:expr $(,)?) => {
        $crate::include_slice!(i64, $file)
    };
}

/// Alias of [`include_slice(i128, path)`](include_slice). Returns a `&'static [i128]`.
#[macro_export]
macro_rules! include_i128s {
    ($file:expr $(,)?) => {
        $crate::include_slice!(i128, $file)
    };
}

/// Alias of [`include_slice(isize, path)`](include_slice). Returns a `&'static [isize]`.
#[macro_export]
macro_rules! include_isizes {
    ($file:expr $(,)?) => {
        $crate::include_slice!(isize, $file)
    };
}

/// Alias of [`include_slice(f32, path)`](include_slice). Returns a `&'static [f32]`.
#[macro_export]
macro_rules! include_f32s {
    ($file:expr $(,)?) => {
        $crate::include_slice!(f32, $file)
    };
}

/// Alias of [`include_slice(f64, path)`](include_slice). Returns a `&'static [f64]`.
#[macro_export]
macro_rules! include_f64s {
    ($file:expr $(,)?) => {
        $crate::include_slice!(f64, $file)
    };
}

/// Force alignment of the `bytes` member to match that of type T.
/// `B` is simply `[u8]` but handles that it is unsized.
#[doc(hidden)]
#[repr(C)]
pub struct AlignedAs<T: bytemuck::Pod, B: Bytes + ?Sized> {
    pub _align: [T; 0],
    pub bytes: B,
}

#[doc(hidden)]
pub trait Bytes {}

impl Bytes for [u8] {}

impl<const N: usize> Bytes for [u8; N] {}
