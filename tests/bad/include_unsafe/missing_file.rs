fn main() {}

// file doesn't exist
static BYTES_31: u128 = unsafe { include_data::include_unsafe!("../../test_data/non-existent") };
