fn main() {}

// Non-Pod type
static BYTES_32_bool: &[bool] = include_data::include_slice!(bool, "../../test_data/binary_32");
