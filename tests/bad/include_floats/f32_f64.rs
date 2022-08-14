use include_data::include_floats;

fn main() {}

// Type mismatch
static BYTES_32_f32_f64: &[f32] = include_floats!(f64, "../../test_data/binary_32");
