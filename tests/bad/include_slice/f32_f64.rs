fn main() {}

// Type mismatch
static BYTES_32_f32_f64: &[f32] = include_data::include_slice!(f64, "../../test_data/binary_32");
