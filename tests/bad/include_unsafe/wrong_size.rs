fn main() {}

static TOO_SHORT: [u16; 30] = unsafe { include_data::include_unsafe!("../../test_data/binary_64") };
