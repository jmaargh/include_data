fn main() {}

// can't make a f64 array of 12 bytes
static BYTES_12: &[f64] = include_data::include_f64!("../../test_data/binary_12");
