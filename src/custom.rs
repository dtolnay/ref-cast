// Not public API. Use #[derive(RefCastCustom)] and #[ref_cast_custom].
#[doc(hidden)]
pub unsafe trait RefCastCustom<From: ?Sized> {
    fn __static_assert() {}
}

pub unsafe trait RefCastOkay<From> {}

unsafe impl<'a, From, To> RefCastOkay<&'a From> for &'a To
where
    From: ?Sized,
    To: ?Sized + RefCastCustom<From>,
{
}

unsafe impl<'a, From, To> RefCastOkay<&'a mut From> for &'a mut To
where
    From: ?Sized,
    To: ?Sized + RefCastCustom<From>,
{
}

pub fn ref_cast_custom<From, To>(_arg: From)
where
    To: RefCastOkay<From>,
{
}
