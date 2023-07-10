# include_data - Include typed data directly in your executable

Sometimes, you want to include data directly in your executable file, but
you don't want to translate that data into Rust code that does the `static`
initialization. This is very useful in embedded contexts, or if you have
some (usually relatively small) data that will always be needed, and you
don't want to deal with loading it from the filesystem and distributing it
as a separate file.

The Rust standard library (and core library) contains
[`include_bytes`](https://doc.rust-lang.org/stable/core/macro.include_bytes.html)
for this purpose. This macro will give you a static reference to a binary array
containing the data from a
file: that is, a `&'static [u8; N]`.

However, if you want to use your static data, you often want it to be of a
particular type, not just a `&[u8]`. For example, you may know that your
included file is a sequence of `f64`s, or a UTF-32 file, or of some
custom type. This crate provides macros for typed compile-time data
includes. This is provided by two main macros:

- `include_data` - outputs any type which is sound
- `include_slice` - outputs a `&'static [T]` slice for any `T` for which
                    this is sound

This crate is `no_std` and also no-`alloc`.

## Stability

While this crate is pre-1.0 you should consider both the API and semantics
unstable. However, this is simply to allow thorough community-review of the
soundness of implementation. I hope that a 1.0 release will come relatively
quickly with few API changes.

## Usage

This library will work out-of-the-box with any type that implements
[`bytemuck::Pod`](https://docs.rs/bytemuck/1.13.1/bytemuck/derive.Pod.html).
This includes:

- Primitive numerical types (`u16`, `i32`, `f64`, etc.)
- Arrays of primitive numerical types (e.g. `[f32; N]`)

For example:
```rust
static MY_INTEGER: i32 = include_data!("../tests/test_data/file_exactly_4_bytes_long");
static SOME_TEXT: &[u32] = include_slice!(u32, "../tests/test_data/some_utf-32_file");
const FOUR_BYTES: [u8; 4] = include_data!("../tests/test_data/file_exactly_4_bytes_long");
```

Note that `include_data` can assign to `const`, while `include_slice` cannot.

Aliases are provided for `include_slice` for primitive number types, using
them is a matter of personal preference. For example:
```rust
static SOME_TEXT: &[u32] = include_u32s!("../tests/test_data/some_utf-32_file");
```

## Usage with custom types

You can include data in any custom type you like. The best way of doing this
is if your custom type satisfies the requirements for
[`bytemuck::Pod`](https://docs.rs/bytemuck/1.13.1/bytemuck/derive.Pod.html),
in which case you can simply use `include_data`.

```rust
#[repr(C)]
#[derive(Copy, Clone)]
struct Foo {
    integer: u16,
    pair: [u8; 2],
}

// Safety: the requirements for `Pod` have been manually checked.
unsafe impl bytemuck::Zeroable for Foo {}
unsafe impl bytemuck::Pod for Foo {}

static FOO_DATA: Foo = include_data!("../tests/test_data/file_exactly_4_bytes_long");
```

Alternatively, if your type cannot implement `bytemuck::Pod` (especially
if it is a foreign type over which you have no control), `include_unsafe`
can be used. In this case, you must guarantee that the file included is
valid for the target type. This may depend on host platform, compiler
version, and compiler profile (amongst other things): recall that Rust does
not have a stable ABI. Clearly, this is **very** unsafe and should be
avoided if possible.

```rust
#[repr(C)]
struct StructWithPadding {
    byte: u8,
    two_bytes: u16,
}

// Safety: we guarantee that the included file contains bytes which are
// a valid bit-pattern for our struct, when compiled on this host.
static BAR_DATA: StructWithPadding = unsafe { include_unsafe!("../tests/test_data/file_exactly_4_bytes_long") };
```

## Safety

All macros exported by this crate are safe, except `include_unsafe`
(assuming, of course, that implementations of `bytemuck::Pod` are sound). If
the input file size does not match the target type (or is not divisible by
it, in the case of slices) or the file cannot be read, compilation will
fail.

`include_unsafe` is **very** unsafe and should only be used with great care.
See the
[documentation](https://docs.rs/include_data/latest/include_data/macro.include_unsafe.html)
for full details.

## Platform-specific behaviour

The interpretation of multi-byte sequences depends on a machine's
endianness. In the case of these macros, multi-byte sequences will be
interpreted into types according to the endianness of the compilation
target, not the compilation host machine.

The interpreation of paths passed to these macros is host-platform specific
and identical to that of
[`include_bytes`](https://doc.rust-lang.org/stable/core/macro.include_bytes.html).

## MSRV

This crate is tested against a fixed version of the Rust compiler (found
in `rust-toolchain.toml`) only so that compiler errors can be consistently
tested. However, all features relied upon were present in Rust 1.0.

## Prior art

The techniques used by this crate were published in a
[blog post by Jack Wrenn](https://jack.wrenn.fyi/blog/include-transmute/).
Some of those techniques were original to Jack, while others were found
in forum threads linked from that post. Please do reach out if you are
somebody involved with these discussions, or have any prior work in this
area. I am also grateful to Jack for comments on an earlier draft of this
crate.