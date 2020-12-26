use ref_cast::RefCast;

#[derive(RefCast)]
#[repr(transparent)]
#[unrecognized]
struct Test {
    s: String,
}

fn main() {}
