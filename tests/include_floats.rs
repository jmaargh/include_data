use include_data::{include_f32, include_f64, include_floats};

#[test]
fn include_f32() {
    static BYTES_32: &[f32] = include_floats!(f32, "test_data/binary_32");
    static BYTES_64: &[f32] = include_f32!("test_data/binary_64");

    assert_eq!(BYTES_32.as_ptr().align_offset(4), 0);
    assert_eq!(BYTES_32.len(), 8);

    assert_eq!(BYTES_64.as_ptr().align_offset(4), 0);
    assert_eq!(BYTES_64.len(), 16);
}

#[test]
fn include_f64() {
    static BYTES_64: &[f64] = include_floats!(f64, "test_data/binary_64");
    static BYTES_32: &[f64] = include_f64!("test_data/binary_32");

    assert_eq!(BYTES_32.as_ptr().align_offset(8), 0);
    assert_eq!(BYTES_32.len(), 4);

    assert_eq!(BYTES_64.as_ptr().align_offset(8), 0);
    assert_eq!(BYTES_64.len(), 8);
}

#[test]
fn bad_use() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/bad/include_floats/*.rs");
}
