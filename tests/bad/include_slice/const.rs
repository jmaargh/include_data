fn main() {}

// Can't use for `const`
const BYTES_32_const: &[u8] = include_data::include_slice!(u8, "../../test_data/binary_32");
