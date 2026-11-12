use implementation::prelude::*;
use rand::Rng;
use rand::seq::IndexedMutRandom;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
pub struct Store(Vec<VecDeque<Ticket>>);

impl Store {
	pub fn new(size: u32, shelf: &mut Shelf, rng: &mut impl Rng) -> Self {
		if !size.is_multiple_of(10) {
			panic!("size must be multiple of 10")
		}

		let mut vec = (0..10)
			.map(|_| VecDeque::<Ticket>::new())
			.collect::<Vec<_>>();

		for _ in 0..(size / 10) {
			let rack: &mut Rack = loop {
				let tmp = shelf.racks_mut().choose_mut(rng).unwrap();

				if tmp.len() == 0 {
					shelf.trim();
					continue;
				} else {
					break tmp;
				}
			};

			for t in rack.take(10) {
				let idx = (t.number() % 10) as usize;
				vec[idx].push_back(t)
			}
		}

		Store(vec)
	}

	pub fn take_10(&mut self) -> Option<[Ticket; 10]> {
		if self.len() < 10 {
			None
		} else {
			Some(std::array::from_fn(|idx| {
				self.0[idx].pop_front().unwrap().clone()
			}))
		}
	}

	pub fn len(&self) -> usize {
		self.0.iter().map(|x| x.len()).sum()
	}

	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new() {
		let mut shelf = Shelf::build(42, 0..100);

		let fixture = Store::new(1_000, &mut shelf, &mut rand::rng());

		for (idx, deque) in fixture.0.iter().enumerate() {
			for t in deque.iter() {
				assert_eq!(t.number() % 10, idx as u32)
			}
		}
	}

	#[test]
	fn take_10() {
		fn assert(actual: Option<&[Ticket; 10]>) {
			let arr = &actual.unwrap();
			assert_eq!(arr.len(), 10);
		}

		let mut shelf = Shelf::build(42, 0..100);
		let mut fixture = Store::new(1_000, &mut shelf, &mut rand::rng());

		for _ in 0..100 {
			let actual = fixture.take_10();
			assert(actual.as_ref())
		}
	}
}
