use include_data::include_ints;

fn main() {}

// Non-integer type
static BYTES_32_bool: &[bool] = include_ints!(bool, "../../test_data/binary_32");
