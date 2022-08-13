use include_data::include_ints;

fn main() {}

// Can't use for `const`
const BYTES_32_const: &[u8] = include_ints!(u8, "../../test_data/binary_32");
