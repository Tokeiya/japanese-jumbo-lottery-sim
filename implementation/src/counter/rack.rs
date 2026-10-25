#![expect(unused_variables, reason = "Under construction")]
#![expect(unfulfilled_lint_expectations, reason = "Under construction")]

use crate::Ticket;
use std::iter::{ExactSizeIterator, FusedIterator, Iterator};
use std::ops::Range;
pub const NUM_BEGIN: u32 = 100_000;
pub const NUM_END: u32 = 200_000;
pub const COUNT: usize = 100_000;

pub struct Rack {
	group: u8,
	begin: u32,
	flg: bool,
	range: Range<u32>,
	cnt: usize,
}

impl Rack {
	pub fn new(group: u8, begin: u32) -> Self {
		if !(NUM_BEGIN..NUM_END).contains(&begin) {
			panic!("Invalid begin value");
		}

		Self {
			group,
			begin,
			flg: false,
			range: begin..NUM_END,
			cnt: 0,
		}
	}
}

impl Iterator for Rack {
	type Item = Ticket;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(i) = self.range.next() {
			self.cnt += 1;
			Some(Ticket::new(self.group, i))
		} else {
			if !self.flg {
				self.flg = true;
				self.range = (NUM_BEGIN..self.begin);

				match self.range.next() {
					Some(i) => {
						self.cnt += 1;
						Some(Ticket::new(self.group, i))
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
		COUNT - self.cnt
	}
}

impl FusedIterator for Rack {}

#[cfg(test)]
mod tests {
	use super::*;
	use std::collections::HashSet;
	use std::panic::catch_unwind;
	#[test]
	fn new() {
		let fixture = Rack::new(100, 100_100);
		assert_eq!(fixture.group, 100);
		assert_eq!(fixture.begin, 100_100);
		assert!(!fixture.flg);
		assert_eq!(fixture.cnt, 0);
	}

	#[test]
	#[should_panic]
	fn invalid_bottom_new() {
		_ = Rack::new(42, 99_999);
	}

	#[test]
	#[should_panic]
	fn invalid_ceil_new() {
		_ = Rack::new(41, NUM_END);
	}

	fn iteration_impl(group: u8, begin: u32) {
		let mut fixture = Rack::new(group, begin);
		assert_eq!(fixture.len(), COUNT);

		let tmp = fixture.next().unwrap();
		assert_eq!(tmp, Ticket::new(group, begin));
		assert_eq!(fixture.len(), COUNT - 1);

		let mut recent = tmp.number();
		let mut cnt = COUNT - 1;

		while let Some(t) = fixture.next() {
			cnt -= 1;
			assert_eq!(fixture.len(), cnt);

			if t.number() == 100_000 {
				assert_eq!(recent, 199_999);
				recent = 100_000;
			} else {
				let ans = catch_unwind(|| t.number() - recent);

				assert_eq!(
					t.number() - recent,
					1,
					"cnt:{cnt} t.num:{} recent:{recent}",
					t.number()
				);

				recent = t.number();
			}
		}

		assert_eq!(fixture.len(), 0);
		//assert_eq!(cnt, 0);
	}

	#[test]
	fn iteration_test() {
		iteration_impl(42, 100_000);
		iteration_impl(45, 110_000);
	}
}
