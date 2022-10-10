use core::marker::PhantomData;
#[cfg(not(no_phantom_pinned))]
use core::marker::PhantomPinned;

pub trait Trivial {}

impl Trivial for () {}
impl<T: ?Sized> Trivial for PhantomData<T> {}

#[cfg(not(no_phantom_pinned))]
impl Trivial for PhantomPinned {}

pub fn assert_trivial<T: Trivial>() {}
