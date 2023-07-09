fn main() {}

static NOT_POD: char = include_data::include_data!("../../test_data/binary_4");

// This type satisfies the requirements of `Pod`, but is not marked as such
#[repr(transparent)]
struct Foo(u16);

static NOT_POD_CUSTOM: Foo = include_data::include_data!("../../test_data/binary_2");
