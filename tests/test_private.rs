use ref_cast::RefCast;

struct PrivateType(usize);

#[derive(RefCast)]
#[repr(transparent)]
pub struct Wrapper(PrivateType);

#[test]
fn test_private() {
    Wrapper::ref_cast(&PrivateType(3));
}
