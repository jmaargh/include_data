fn main() {}

static NOT_ABP: char = include_data::include_data!("../../test_data/binary_4");

// This type satisfies the requirements of `AnyBitPattern`, but is not marked as such
#[repr(transparent)]
struct Foo(u16);

static NOT_ABP_CUSTOM: Foo = include_data::include_data!("../../test_data/binary_2");
