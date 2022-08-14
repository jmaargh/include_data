use include_data::include_ints;

fn main() {}

// can't make a u16 array of 31 bytes
static BYTES_31: &[u16] = include_ints!(u16, "../../test_data/binary_31");
