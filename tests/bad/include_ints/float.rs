use include_data::include_ints;

fn main() {}

// Can't use this macro for floats
static FLOATS: &[f32] = include_ints!(f32, "../../test_data/binary_64");
