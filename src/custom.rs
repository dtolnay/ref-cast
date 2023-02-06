// Not public API. Use #[derive(RefCastCustom)] and #[ref_cast_custom].
#[doc(hidden)]
pub unsafe trait RefCastCustom<From: ?Sized> {
    type CurrentCrate;
    fn __static_assert() {}
}

unsafe impl<From, To> RefCastCustom<[From]> for [To]
where
    To: RefCastCustom<From>,
{
    type CurrentCrate = To::CurrentCrate;
    fn __static_assert() {
        To::__static_assert();
    }
}

pub unsafe trait RefCastOkay<From>: Sealed<From> {
    type CurrentCrate;
    type Target: ?Sized;
}

unsafe impl<'a, From, To> RefCastOkay<&'a From> for &'a To
where
    From: ?Sized,
    To: ?Sized + RefCastCustom<From>,
{
    type CurrentCrate = To::CurrentCrate;
    type Target = To;
}

unsafe impl<'a, From, To> RefCastOkay<&'a mut From> for &'a mut To
where
    From: ?Sized,
    To: ?Sized + RefCastCustom<From>,
{
    type CurrentCrate = To::CurrentCrate;
    type Target = To;
}

pub trait Sealed<From> {}

impl<'a, From, To> Sealed<&'a From> for &'a To
where
    From: ?Sized,
    To: ?Sized + RefCastCustom<From>,
{
}

impl<'a, From, To> Sealed<&'a mut From> for &'a mut To
where
    From: ?Sized,
    To: ?Sized + RefCastCustom<From>,
{
}

pub type CurrentCrate<From, To> = <To as RefCastOkay<From>>::CurrentCrate;

pub fn ref_cast_custom<From, To>(_arg: From)
where
    To: RefCastOkay<From>,
{
}
