fn main() {}

// file doesn't exist
static BYTES_31: u128 = include_data::include_data!("../../test_data/non-existent");
