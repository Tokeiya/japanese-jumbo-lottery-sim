use super::rack::Rack;
use crate::Ticket;
use std::ops::{Index, IndexMut, RangeBounds};

pub struct Shelf {
	racks: Vec<Rack>,
}

fn todo<T>() -> T {
	todo!()
}
impl Shelf {
	pub fn build(range: impl RangeBounds<u8>) -> Self {
		todo!()
	}

	pub fn build_with_rand(range: impl RangeBounds<u8>, rnd: &mut impl rand::Rng) -> Self {
		todo!()
	}

	pub fn iter(&self) -> impl ExactSizeIterator + std::iter::FusedIterator {
		todo::<std::slice::Iter<'_, Rack>>()
	}

	pub fn iter_mut(&mut self) -> impl ExactSizeIterator + std::iter::FusedIterator {
		todo::<std::slice::IterMut<'_, Rack>>()
	}

	pub fn trim(&mut self) -> Vec<Ticket> {
		todo!()
	}
}

impl Index<usize> for Shelf {
	type Output = Rack;

	fn index(&self, index: usize) -> &Self::Output {
		todo!()
	}
}

impl IndexMut<usize> for Shelf {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use rand::Rng;
	use rand::distr::uniform::{Error, SampleRange};
	use std::cell::Cell;
	use std::collections::HashSet;

	#[test]
	fn build() {
		let fixture = Shelf::build(1..=100);

		assert_eq!(fixture.racks.len(), 100);

		for (i, r) in fixture.racks.iter().enumerate() {
			assert_eq!(r.group(), (i + 1) as u8);
			assert_eq!(r.len(), 100_000);
		}
	}

	#[test]
	fn build_with_rand() {
		let mut rng = rand::rng();
		let mut set: HashSet<u32, _> = HashSet::new();

		let fixture = Shelf::build_with_rand(1..=100, &mut rng);

		todo()
	}
}
