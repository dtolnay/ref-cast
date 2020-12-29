use core::{mem, usize};

pub struct Layout<T: ?Sized>(T);

pub trait LayoutUnsized<T: ?Sized> {
    const SIZE: usize = usize::MAX;
    const ALIGN: usize = usize::MAX;
}

impl<T: ?Sized> LayoutUnsized<T> for Layout<T> {}

impl<T> Layout<T> {
    pub const SIZE: usize = mem::size_of::<T>();
    pub const ALIGN: usize = mem::align_of::<T>();
}

#[inline]
pub fn assert_layout<Outer: ?Sized, Inner: ?Sized>(
    outer_size: usize,
    inner_size: usize,
    outer_align: usize,
    inner_align: usize,
) {
    if outer_size != inner_size {
        panic!("unexpected size in cast: {} != {}", outer_size, inner_size);
    }
    if outer_align != inner_align {
        panic!(
            "unexpected alignment in cast: {} != {}",
            outer_align, inner_align,
        );
    }
}
