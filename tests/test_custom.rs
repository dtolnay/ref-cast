use ref_cast::{ref_cast_custom, RefCastCustom};

#[derive(RefCastCustom)]
#[repr(transparent)]
pub struct Example(u32);

impl Example {
    #[ref_cast_custom]
    const fn new_slice(ints: &[u32]) -> &[Self];
}

#[test]
fn test_slice() {
    let example = Example::new_slice(&[3, 2, 1]);
    assert_eq!(example.len(), 3);
    assert_eq!(example[0].0, 3);
    assert_eq!(example[1].0, 2);
    assert_eq!(example[2].0, 1);
}
