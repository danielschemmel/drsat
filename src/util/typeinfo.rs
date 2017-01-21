use ::std::fmt;
use ::std::marker::PhantomData;
use ::std::mem::{size_of, align_of};

pub struct Typeinfo<T> {
	phantom: PhantomData<T>,
}

impl<T> Typeinfo<T> {
	pub fn new() -> Self {
		Typeinfo{ phantom: PhantomData }
	}

	pub fn size() -> usize {
		size_of::<T>()
	}

	pub fn alignment() -> usize {
		align_of::<T>()
	}
}

impl<T> fmt::Display for Typeinfo<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let width = f.width().unwrap_or(0);
		write!(f, "size: {1:0$} B, alignment: {2:0$} B", width, Self::size(), Self::alignment())
	}
}