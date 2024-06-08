use include_data::include_unsafe;

#[test]
fn with_padding() {
    #[repr(C)]
    struct StructWithBools {
        bool1: bool,
        bool2: bool,
        two_bytes: u16,
    }

    // Safety: we guarantee that the included file contains the bytes 0x00 and
    // 0x01 at the start, which are both valid bool bit patterns.
    static BAR_DATA: StructWithBools =
        unsafe { include_unsafe!("../tests/test_data/file_exactly_4_bytes_long") };

    assert_eq!(core::mem::size_of::<StructWithBools>(), 4);
    assert_eq!(BAR_DATA.bool1, true);
    assert_eq!(BAR_DATA.bool2, false);
    if cfg!(target_endian = "little") {
        assert_eq!(BAR_DATA.two_bytes, 0x0302);
    } else {
        assert_eq!(BAR_DATA.two_bytes, 0x0203);
    }

    let data_transmuted: [u8; 4] = unsafe { core::mem::transmute_copy(&BAR_DATA) };

    assert_eq!(data_transmuted, [0x01, 0x00, 0x02, 0x03]);
}

#[test]
fn include_as_const() {
    const MY_U32: u32 = unsafe { include_unsafe!("test_data/binary_4") };

    if cfg!(target_endian = "little") {
        assert_eq!(MY_U32, 0x03_02_01_00);
    } else {
        assert_eq!(MY_U32, 0x00_01_02_03);
    }
}

#[test]
fn bad_use() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/bad/include_unsafe/*.rs");
}
