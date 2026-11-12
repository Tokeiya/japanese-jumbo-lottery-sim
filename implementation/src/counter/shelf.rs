use super::rack::Rack;
use rand::RngExt;
use std::ops::{Index, IndexMut, RangeBounds};

pub struct Shelf {
	racks: Vec<Rack>,
}

// fn todo<T>() -> T {
// 	todo!()
// }
impl Shelf {
	pub fn build(unit: u8, range: impl RangeBounds<u8> + Iterator<Item = u8>) -> Self {
		let mut vec = Vec::new();

		for grp in range {
			vec.push(Rack::new(unit, grp, super::NUM_BEGIN))
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
			let begin = rnd.random_range(super::NUM_BEGIN / 10..super::NUM_END / 10) * 10;
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

	pub fn len(&self) -> usize {
		self.racks.iter().map(|r| r.len()).sum()
	}

	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	pub fn trim(&mut self) {
		self.racks.retain(|x| x.len() != 0)
	}

	pub fn racks(&self) -> &[Rack] {
		self.racks.as_slice()
	}

	pub fn racks_mut(&mut self) -> &mut [Rack] {
		self.racks.as_mut()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::Ticket;
	use std::collections::HashSet;

	const GROUP_SIZE: usize = 100;
	const NUMBER_BEGIN: u32 = 100_000;

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
	fn racks() {
		let fixture = generate();
		assert!(std::ptr::eq(
			fixture.racks.as_ptr(),
			fixture.racks().as_ptr()
		));
	}

	#[test]
	fn racks_mut() {
		let mut fixture = generate();

		assert!(std::ptr::eq(
			fixture.racks_mut().as_ptr(),
			fixture.racks.as_ptr()
		));
	}

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

	#[test]
	fn len_is_empty() {
		let mut fixture = generate();

		assert_eq!(fixture.len(), 10_000_000);
		assert!(!fixture.is_empty());

		let mut cnt = 10_000_000usize;

		loop {
			if fixture.racks[0].next().is_some() {
				cnt -= 1;
				continue;
			} else {
				fixture.trim();

				if fixture.racks.is_empty() {
					break;
				}
			}

			assert_eq!(fixture.len(), cnt);
			assert!(!fixture.is_empty());
		}

		assert_eq!(fixture.len(), 0);
		assert!(fixture.is_empty());
	}

	#[test]
	fn trim() {
		let mut fixture = generate();

		assert_eq!(fixture.racks.len(), GROUP_SIZE);
		for i in 0..GROUP_SIZE {
			fixture.trim();

			if fixture.racks.is_empty() {
				break;
			}

			assert_eq!(fixture.racks.len(), GROUP_SIZE - i);

			for _ in fixture.racks[0].by_ref() {}
		}

		fixture.trim();
		assert_eq!(fixture.racks.len(), 0);
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
}
