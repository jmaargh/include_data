use include_data::include_f32;

fn main() {}

// Can't use this macro for ints, even of the same size
static INTS: &[u32] = include_f32!("../../test_data/binary_64");
