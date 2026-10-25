use std::assert_matches;
use std::ops::Range;
use std::ops::RangeBounds;

const NUM_THRESHOLD: u32 = 200_000;
const CNT_THRESHOLD: usize = 100_000;

pub struct Rack {
	begin: u32,
	flg: bool,
	range: Range<u32>,
	cnt: usize,
}

impl Rack {
	pub fn new(begin: u32) -> Self {
		Self {
			begin,
			flg: false,
			range: (begin..NUM_THRESHOLD),
			cnt: 0,
		}
	}
}

impl Iterator for Rack {
	type Item = u32;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(i) = self.range.next() {
			self.cnt += 1;
			Some(i)
		} else {
			if !self.flg {
				self.flg = true;
				self.range = (100_000..self.begin);

				match self.range.next() {
					Some(i) => {
						self.cnt += 1;
						Some(i)
					}
					None => None,
				}
			} else {
				None
			}
		}
	}
}

impl ExactSizeIterator for Rack {
	fn len(&self) -> usize {
		CNT_THRESHOLD - self.cnt
	}
}

impl std::iter::FusedIterator for Rack {}

fn main() {
	let mut rng = (100_000..NUM_THRESHOLD);
	let mut recent = 0u32;
	let mut cnt = 0usize;

	while let Some(i) = rng.next() {
		cnt += 1;
		println!("{cnt}:{i}");
	}

	dbg!(rng.len());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn len() {
		let mut rack = Rack::new(100_000);

		for i in 0..CNT_THRESHOLD {
			assert_eq!(rack.len(), CNT_THRESHOLD - i);
			_ = rack.next().unwrap();
		}
		assert_eq!(rack.len(), 0);
		assert_eq!(rack.next(), None);

		let mut rack = Rack::new(100_100);
		for i in 0..CNT_THRESHOLD {
			assert_eq!(rack.len(), CNT_THRESHOLD - i);
			_ = rack.next().unwrap();
		}
		assert_eq!(rack.len(), 0);
		assert_eq!(rack.next(), None);
	}
}
