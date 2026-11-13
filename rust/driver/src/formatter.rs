use std::fmt::{Display, Formatter};

pub trait ThousandsSeparator {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result;

	fn separated(&self) -> Separated<'_, Self>
	where
		Self: Sized,
	{
		Separated(self)
	}
}

pub struct Separated<'a, T: ?Sized>(&'a T);

impl<T: ThousandsSeparator + ?Sized> Display for Separated<'_, T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		ThousandsSeparator::fmt(self.0, f)
	}
}
