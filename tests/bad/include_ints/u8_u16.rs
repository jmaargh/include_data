use include_data::include_ints;

fn main() {}

// Type mismatch
static BYTES_32_u8_u16: &[u8] = include_ints!(u16, "../../test_data/binary_32");
