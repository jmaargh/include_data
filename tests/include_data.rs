use include_data::include_data;

#[test]
fn include_u32() {
    static MY_U32: u32 = include_data!("test_data/binary_4");

    if cfg!(target_endian = "little") {
        assert_eq!(MY_U32, 0x03_02_01_00);
    } else {
        assert_eq!(MY_U32, 0x00_01_02_03);
    }
}

#[test]
fn include_utf32() {
    static LOREM_IPSUM: [u32; 13] = include_data!("test_data/lorem_ipsum_utf32");

    const REFERENCE: &str = "Lorem ipsum\n";

    // N.B. file starts with a byte order mark which is skipped
    for (i, c) in REFERENCE.char_indices() {
        assert_eq!(c as u32, LOREM_IPSUM[i + 1]);
    }
}

#[test]
fn include_custom() {
    #[repr(C)]
    #[derive(Copy, Clone)]
    struct Foo {
        integer: u16,
        pair: (u8, u8),
    }

    // Safety: the type `Foo` has been checked to satisfy all requirements of
    // `Pod`.
    unsafe impl bytemuck::Zeroable for Foo {}
    unsafe impl bytemuck::Pod for Foo {}

    static FOO_DATA: Foo = include_data!("../tests/test_data/binary_4");

    assert_eq!(FOO_DATA.integer, 0x0100);
    assert_eq!(FOO_DATA.pair, (0x02, 0x03));
}

#[test]
fn include_as_const() {
    const MY_U32: u32 = include_data!("test_data/binary_4");

    if cfg!(target_endian = "little") {
        assert_eq!(MY_U32, 0x03_02_01_00);
    } else {
        assert_eq!(MY_U32, 0x00_01_02_03);
    }
}

#[test]
fn const_custom() {
    #[repr(C)]
    #[derive(Copy, Clone)]
    struct Foo {
        integer: u16,
        pair: (u8, u8),
    }

    // Safety: the type `Foo` has been checked to satisfy all requirements of
    // `Pod`.
    unsafe impl bytemuck::Zeroable for Foo {}
    unsafe impl bytemuck::Pod for Foo {}

    const FOO_DATA: Foo = include_data!("../tests/test_data/binary_4");

    assert_eq!(FOO_DATA.integer, 0x0100);
    assert_eq!(FOO_DATA.pair, (0x02, 0x03));
}

#[test]
fn bad_use() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/bad/include_data/*.rs");
}
