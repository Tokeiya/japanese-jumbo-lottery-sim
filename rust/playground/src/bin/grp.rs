use rand::prelude::*;
use rand::seq::SliceRandom;
struct RandShelf([Vec<u8>; 10], usize);

impl Default for RandShelf {
	fn default() -> Self {
		let mut rng = rand::rng();

		Self(
			std::array::from_fn(|_| {
				let mut v = (1u8..=100)
					.flat_map(|i| (0..1000000).map(move |_| i))
					.collect::<Vec<u8>>();
				v.shuffle(&mut rng);
				v
			}),
			0,
		)
	}
}

impl RandShelf {
	pub fn take_10(&mut self) -> Option<[u8; 10]> {
		if self.1 == 0 {
			None
		} else {
			self.1 -= 10;
			Some(std::array::from_fn(|i| unsafe {
				self.0[i].pop().unwrap_unchecked()
			}))
		}
	}
}

fn batch(agg: &mut [usize; 10]) {
	let mut shelf = RandShelf::default();
	let agg = [0usize; 10];
	for _ in 0..1_000_000 {
		let mut tmp = [0u8; 100];
	}
}

fn main() {}
