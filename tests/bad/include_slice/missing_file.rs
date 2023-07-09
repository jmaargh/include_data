fn main() {}

// file doesn't exist
static BYTES_31: &[i32] = include_data::include_slice!(i32, "../../test_data/non-existent");
