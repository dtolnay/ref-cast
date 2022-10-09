use ref_cast::RefCast;

struct PrivateType(usize);

#[derive(RefCast)]
#[repr(transparent)]
pub struct WrapPrivateType(PrivateType);

#[test]
fn test_private_type() {
    WrapPrivateType::ref_cast(&PrivateType(3));
}

mod inner {
    #[derive(super::RefCast, Debug)]
    #[repr(transparent)]
    pub struct EvenNumber(usize);

    impl EvenNumber {
        pub fn try_ref_cast(input: &usize) -> Option<&EvenNumber> {
            if input % 2 == 0 {
                Some(Self::ref_cast(input))
            } else {
                None
            }
        }
    }

    #[derive(super::RefCast)]
    #[repr(transparent)]
    pub struct PubCrateWrap(pub(crate) usize);
}

#[test]
fn test_inner_type() {
    // error[E0624]: associated function `ref_cast` is private
    // inner::EvenNumber::ref_cast(&4);

    inner::EvenNumber::try_ref_cast(&4).unwrap();
    inner::EvenNumber::try_ref_cast(&5).ok_or(()).expect_err("not an even number");
    inner::PubCrateWrap::ref_cast(&5);
}
