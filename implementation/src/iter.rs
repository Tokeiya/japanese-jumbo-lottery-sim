use std::range::{Range, RangeInclusive};

pub trait Iter<T> {
	fn iter(&self) -> impl Iterator<Item = T>;
}

impl<T> Iter<T> for Range<T>
where
	Range<T>: Clone + IntoIterator<Item = T>,
{
	fn iter(&self) -> impl Iterator<Item = T> {
		self.clone().into_iter()
	}
}

impl<T> Iter<T> for RangeInclusive<T>
where
	RangeInclusive<T>: Clone + IntoIterator<Item = T>,
{
	fn iter(&self) -> impl Iterator<Item = T> {
		self.clone().into_iter()
	}
}
