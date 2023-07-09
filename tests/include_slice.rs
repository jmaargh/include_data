#![allow(clippy::modulo_one)]

use include_data::{include_slice, include_u16};

#[test]
fn include_bytes() {
    static BYTES_32: &[u8] = include_slice!(u8, "test_data/binary_32");

    assert_eq!(BYTES_32.len(), 32);
    for i in 0..32 {
        assert_eq!(BYTES_32[i as usize], i);
    }
}

#[test]
fn include_u16() {
    static BYTES_32: &[u16] = include_slice!(u16, "test_data/binary_32");
    static BYTES_64: &[u16] = include_u16!("test_data/binary_64");

    assert_eq!(BYTES_32.as_ptr().align_offset(2), 0);
    assert_eq!(BYTES_32.len(), 16);
    for i in 0..16 {
        let i2 = (i as u16) * 2;
        assert_eq!(BYTES_32[i], i2 + ((i2 + 1) << 8));
    }

    assert_eq!(BYTES_32.as_ptr().align_offset(2), 0);
    assert_eq!(BYTES_64.len(), 32);
}

#[test]
fn include_u32() {
    static BYTES_32: &[u32] = include_slice!(u32, "test_data/binary_32");

    assert_eq!(BYTES_32.as_ptr().align_offset(4), 0);
    assert_eq!(BYTES_32.len(), 8);
    for i in 0..8 {
        let i4 = (i as u32) * 4;
        assert_eq!(
            BYTES_32[i],
            i4 + ((i4 + 1) << 8) + ((i4 + 2) << 16) + ((i4 + 3) << 24),
        );
    }
}

#[test]
fn include_31_u8s() {
    static BYTES_31: &[u8] = include_slice!(u8, "test_data/binary_31");

    assert_eq!(BYTES_31.as_ptr().align_offset(1), 0);
    assert_eq!(BYTES_31.len(), 31);
    for i in 0..31 {
        assert_eq!(BYTES_31[i as usize], i);
    }
}

#[test]
fn include_i128() {
    static BYTES_64: &[i128] = include_slice!(i128, "test_data/binary_64");

    assert_eq!(BYTES_64.as_ptr().align_offset(8), 0);
    assert_eq!(BYTES_64.len(), 4);
    for i in 0..4 {
        let i16 = (i as i128) * 16;
        let mut val = 0;
        for j in 0..16 {
            val += (i16 + j) << (8 * j);
        }

        assert_eq!(BYTES_64[i], val);
    }
}

#[test]
fn bad_use() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/bad/include_slice/*.rs");
}

mod noincs {
    #[test]
    fn no_include_i32() {
        static INTS: &[i32] = include_data::include_i32!("test_data/binary_32");

        assert_eq!(INTS.as_ptr().align_offset(4), 0);
        assert_eq!(INTS.len(), 8);
    }
}
