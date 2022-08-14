use include_data::include_floats;

fn main() {}

// file doesn't exist
static BYTES_31: &[f64] = include_floats!(f64, "../../test_data/non-existent");
