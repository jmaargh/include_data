//! Macros in this module are all `unsafe` and require `unsafe` blocks.

/// Unsafe. Includes a file as a static reference to any given sized type.
///
/// The return type is `&'static T`, where `T` is inferred. The binary
/// representation of the included `T` value is taken from the specified file.
///
/// Uses [`std::include_bytes`](std::include_bytes) and therefore has the same
/// portability limitations on the path.
///
/// Originally from [Jack Wrenn](https://jack.wrenn.fyi/blog/include-transmute/).
///
/// # Safety
///
/// This is **deeply** unsafe and requires an `unsafe` block. It is highly
/// recommended that the safe `include_*` macros are preferred over this
/// unless the user is certain they know what they are doing.
///
/// A user must ensure that the file contains a bit pattern that is valid as a
/// rust value of the target type, on the target architecture, as interpreted
/// by the build host's toolchain. Note that since Rust lacks a stable ABI,
/// most target types (certainly structs and enums) should be marked
/// `#[repr(C)]` so the bit representation is guaranteed.
///
/// Alignment is not an issue, as this macro
///
/// [incb]: std::include_bytes
///
/// # Example
///
/// ```rust
/// use include_data::include_sized;
///
/// #[repr(C)]
/// struct SimpleStruct {
///     pub a: u8,
///     pub b: u8,
/// }
///
/// // SAFETY: binary_2 is exactly 2 bytes long and `SimpleStruct` is valid for
/// // any 2 bytes
/// static SIMPLE_STATIC: &SimpleStruct =  unsafe { include_sized!("../tests/test_data/binary_2") };
///
/// // binary_2 contains a 0 byte followed by a 1 byte
/// assert_eq!(SIMPLE_STATIC.a, 0);
/// assert_eq!(SIMPLE_STATIC.b, 1);
/// ```
#[macro_export]
macro_rules! include_sized {
    ($file:expr $(,)?) => {
        &::core::mem::transmute(*include_bytes!($file))
    };
}
