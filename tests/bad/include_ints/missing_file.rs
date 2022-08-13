use include_data::include_ints;

fn main() {}

// file doesn't exist
static BYTES_31: &[i32] = include_ints!(i32, "../../test_data/non-existent");
