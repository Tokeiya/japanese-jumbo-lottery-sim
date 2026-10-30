use super::rack::Rack;
use crate::Ticket;
use rand::RngExt;
use std::ops::{Index, IndexMut, RangeBounds};

const NUMBER_BEGIN: u32 = 100_000;

pub struct Shelf {
	racks: Vec<Rack>,
}

fn todo<T>() -> T {
	todo!()
}
impl Shelf {
	pub fn build(unit: u8, range: impl RangeBounds<u8> + Iterator<Item = u8>) -> Self {
		let mut vec = Vec::new();

		for grp in range {
			vec.push(Rack::new(unit, grp, NUMBER_BEGIN))
		}

		Self { racks: vec }
	}

	pub fn build_with_rand(
		unit: u8,
		range: impl RangeBounds<u8> + Iterator<Item = u8>,
		rnd: &mut impl rand::Rng,
	) -> Self {
		let mut vec = Vec::new();

		for grp in range {
			let begin = rnd.random_range(10_000..20_000) * 10;
			vec.push(Rack::new(unit, grp, begin));
		}

		Self { racks: vec }
	}

	pub fn iter(&self) -> impl ExactSizeIterator<Item = &Rack> + std::iter::FusedIterator {
		self.racks.iter()
	}

	pub fn iter_mut(
		&mut self,
	) -> impl ExactSizeIterator<Item = &mut Rack> + std::iter::FusedIterator {
		self.racks.iter_mut()
	}
}

impl Index<usize> for Shelf {
	type Output = Rack;

	fn index(&self, index: usize) -> &Self::Output {
		&self.racks[index]
	}
}

impl IndexMut<usize> for Shelf {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.racks[index]
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::collections::HashSet;

	const GROUP_SIZE: usize = 100;
	const NUMBER_BEGIN: u32 = 100_000;

	#[test]
	fn build() {
		let fixture = Shelf::build(10, 1..=GROUP_SIZE as u8);

		assert_eq!(fixture.racks.len(), GROUP_SIZE);

		for (i, r) in fixture.racks.iter().enumerate() {
			assert_eq!(r.group(), (i + 1) as u8);
			assert_eq!(r.len(), 100_000);
			assert_eq!(r.begin(), NUMBER_BEGIN);
		}
	}

	#[test]
	fn build_with_rand() {
		let mut rng = rand::rng();
		let mut set: HashSet<Ticket, _> = HashSet::new();
		let mut fixture = Shelf::build_with_rand(42, 1..=GROUP_SIZE as u8, &mut rng);

		for rack in fixture.racks.iter_mut() {
			for ticket in rack {
				assert!(set.insert(ticket));
			}

			assert_eq!(set.len(), 100_000);
			set.clear();
		}
	}

	fn generate() -> Shelf {
		let mut vec = Vec::new();

		let mut begin = 100_000;

		for i in 1..=100u8 {
			vec.push(Rack::new(20, i, begin));
			begin += 1_000;
		}

		Shelf { racks: vec }
	}

	#[test]
	fn iter_test() {
		let fixture = generate();

		let mut exp_begin = 100_000;
		for (i, r) in fixture.iter().enumerate() {
			assert_eq!(r.group(), (i + 1) as u8);
			assert_eq!(r.begin(), exp_begin);
			exp_begin += 1_000;
		}
	}

	#[test]
	fn iter_mut_test() {
		let mut fixture = generate();

		let mut exp_begin = 100_000;
		for (i, r) in fixture.iter_mut().enumerate() {
			assert_eq!(r.group(), (i + 1) as u8);
			assert_eq!(r.begin(), exp_begin);
			let ticket = r.next().unwrap();
			assert_eq!(ticket.number(), exp_begin);
			exp_begin += 1_000;
		}
	}

	#[test]
	fn index_test() {
		let fixture = generate();

		let mut exp_num = 100_000;

		for i in 0..GROUP_SIZE {
			let rack = &fixture[i];
			assert_eq!(rack.group(), (i + 1) as u8);
			assert_eq!(rack.begin(), exp_num);
			exp_num += 1_000;
		}
	}

	#[test]
	fn index_mut_test() {
		let mut fixture = generate();

		let mut exp_num = 100_000;

		for i in 0..GROUP_SIZE {
			let rack = &mut fixture[i];
			assert_eq!(rack.group(), (i + 1) as u8);
			assert_eq!(rack.begin(), exp_num);
			let ticket = rack.next().unwrap();
			assert_eq!(ticket.number(), exp_num);

			exp_num += 1_000;
		}
	}
}
