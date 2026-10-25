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

	pub fn build_with_rand(range: impl RangeBounds<u8>, rnd: impl rand::Rng) -> Self {
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
	use mockall::{concretize, mock};
	use rand::Rng;
	use rand::distr::uniform::{Error, SampleRange};
	use std::cell::Cell;

	struct Dummy(Cell<u32>, Cell<usize>);

	impl Dummy {
		pub fn new() -> Self {
			Self(Cell::new(0), Cell::new(0))
		}

		pub fn call_count(&self) -> usize {
			self.1.get()
		}
	}

	impl SampleRange<u32> for Dummy {
		fn sample_single<R: Rng + ?Sized>(self, _: &mut R) -> Result<u32, Error> {
			todo!()
		}

		fn is_empty(&self) -> bool {
			false
		}
	}

	#[test]
	fn build() {
		let fixture = Shelf::build(1..=100);

		assert_eq!(fixture.racks.len(), 100);

		for (i, r) in fixture.racks.iter().enumerate() {
			assert_eq!(r.group(), (i + 1) as u8);
			assert_eq!(r.len(), 100_000);
		}
	}
}
