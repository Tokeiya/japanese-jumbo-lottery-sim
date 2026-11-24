use implementation::prelude::*;
use rand::Rng;
use rand::seq::IndexedMutRandom;
use rand::seq::SliceRandom;
use std::collections::VecDeque;
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

		for elem in vec.iter_mut() {
			elem.make_contiguous().shuffle(rng);
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

pub fn take_sequence(shelf: &mut Shelf, rng: &mut impl Rng) -> [Ticket; 10] {
	let mut cnt = 0usize;
	let grp = loop {
		shelf.trim();
		let tmp = shelf.racks_mut().choose_mut(rng).unwrap();
		if tmp.len() < 10 {
			cnt += 1;
			assert!(cnt > 100, "Reach Threshold");
			continue;
		} else {
			break tmp;
		}
	};

	std::array::from_fn(|_| grp.next().unwrap())
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::collections::HashSet;

	#[test]
	fn take_sequence_test() {
		let mut rng = rand::rng();
		let mut shelf = Shelf::build_with_rand(42, 0..=100, &mut rng);

		for _ in 0..100 {
			let actual = take_sequence(&mut shelf, &mut rng);
			let mut iter = actual.iter();
			let t = iter.next().unwrap();

			let grp = t.group();
			assert_eq!(t.number() % 10, 0);
			let mut recent = t.number();

			for t in iter {
				assert_eq!(t.group(), grp);
				assert_eq!(t.number(), recent + 1);
				recent += 1;
			}
		}
	}

	#[test]
	fn len_is_empty() {
		let mut shelf = Shelf::build(42, 0..100);
		let mut fixture = Store::new(1_000, &mut shelf, &mut rand::rng());

		assert_eq!(fixture.len(), 1_000);
		assert!(!fixture.is_empty());

		let mut expected = 1_000;

		for idx in 0..fixture.0.len() {
			while !fixture.0[idx].pop_front().is_none() {
				expected -= 1;
				assert_eq!(fixture.len(), expected);

				if expected != 0 {
					assert!(!fixture.is_empty());
				}
			}
		}

		assert_eq!(0, fixture.len());
		assert!(fixture.is_empty());
	}

	#[test]
	fn new() {
		let mut shelf = Shelf::build(42, 0..100);
		let fixture = Store::new(1_000, &mut shelf, &mut rand::rng());

		assert_eq!(shelf.iter().map(|x| x.len()).sum::<usize>(), 9_999_000);

		assert_eq!(fixture.0.iter().map(|x| x.len()).sum::<usize>(), 1_000usize);
		assert_eq!(1_000usize, fixture.0.iter().map(|x| x.len()).sum::<_>());

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

	#[test]
	fn take_10_digit_seq() {
		let mut rng = rand::rng();
		let mut shelf = Shelf::build_with_rand(42, 0..=100, &mut rng);
		let mut fixture = Store::new(1000, &mut shelf, &mut rng);

		let actual = fixture.take_10().unwrap();

		let mut num = HashSet::new();
		let mut grp = HashSet::new();

		for i in 0..10 {
			assert_eq!(actual[i].number() % 10, i as u32);
			num.insert(actual[i].number() / 10);
			grp.insert(actual[i].group());

			println!("{i}:{:?}", actual[i]);
		}

		assert!(num.len() >= 8);
		assert!(grp.len() >= 8);
	}
}
