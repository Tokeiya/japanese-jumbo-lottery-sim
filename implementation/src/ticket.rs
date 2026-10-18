use rand::{Rng, RngExt};
use std::ops::RangeBounds;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
	#[error("Invalid group range")]
	InvalidGroupRange,
	#[error("Invalid number range")]
	InvalidNumberRange,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Ticket {
	group: u8,
	number: u32,
}

fn range_check<Idx: PartialEq + Eq + PartialOrd + Ord>(range: impl RangeBounds<Idx>) {
	todo!()
}

impl Ticket {
	pub fn try_from_rng(
		group_range: impl RangeBounds<u8>,
		number_range: impl RangeBounds<u32>,
		rng: &mut impl Rng,
	) -> Result<Self> {
		todo!()
	}

	pub fn new(group: u8, number: u32) -> Self {
		todo!()
	}

	pub fn group(&self) -> u8 {
		todo!()
	}

	pub fn number(&self) -> u32 {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use core::convert::Infallible;
	use mockall::mock;
	use rand::SeedableRng;
	use rand::TryRng;
	use rand::rngs::Xoshiro256PlusPlus;
	use std::assert_matches;
	use std::collections::VecDeque;

	mock! {
		pub Rng {}

		impl TryRng for Rng {
			type Error = Infallible;

			fn try_next_u32(&mut self) -> std::result::Result<u32, Infallible>;
			fn try_next_u64(&mut self) -> std::result::Result<u64, Infallible>;
			fn try_fill_bytes(&mut self, dest: &mut [u8]) -> std::result::Result<(), Infallible>;
		}
	}

	#[test]
	fn foo() {
		let mut rng = Xoshiro256PlusPlus::seed_from_u64(114514);
		print!("vec![");
		for _ in 0..100 {
			print!("{}u32,", rng.next_u32());
		}
		print!("]");
	}

	#[test]
	fn try_from_rng() {
		let mut act = Xoshiro256PlusPlus::from_rng(&mut rand::rng());
		let mut exp = act.clone();

		for _ in 0..10000 {
			let fixture = Ticket::try_from_rng(1u8..=100, 1u32..=10_000, &mut act).unwrap();
			assert_eq!(fixture.group, exp.random_range(1u8..=100));
			assert_eq!(fixture.number, exp.random_range(1u32..=10_000));
		}
	}

	#[test]
	fn invalid_try_from_rng() {
		let mut mock = MockRng::new();
		assert_matches!(
			Ticket::try_from_rng(0..0, 0..100, &mut mock),
			Err(Error::InvalidGroupRange)
		);

		assert_matches!(
			Ticket::try_from_rng(100..0, 0..100, &mut mock),
			Err(Error::InvalidGroupRange)
		);

		assert_matches!(
			Ticket::try_from_rng(1..100, 0..0, &mut mock),
			Err(Error::InvalidNumberRange)
		);

		assert_matches!(
			Ticket::try_from_rng(1..100, 1000..100, &mut mock),
			Err(Error::InvalidNumberRange)
		);
	}

	#[test]
	fn new() {
		for g in 1u8..10 {
			for n in 1u32..100 {
				let fixture = Ticket::new(g, n);
				assert_eq!(fixture.number, n);
				assert_eq!(fixture.group, g);
			}
		}
	}

	#[test]
	fn number() {
		let fixture = Ticket {
			number: 114514,
			group: 19,
		};
	}
}
