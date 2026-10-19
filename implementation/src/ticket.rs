use rand::{Rng, RngExt, distr::uniform::SampleRange};
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

impl Ticket {
	pub fn try_from_rng(
		group_range: impl SampleRange<u8>,
		number_range: impl SampleRange<u32>,
		rng: &mut impl Rng,
	) -> Result<Self> {
		if group_range.is_empty() {
			Err(Error::InvalidGroupRange)
		} else if number_range.is_empty() {
			Err(Error::InvalidNumberRange)
		} else {
			Ok(Self::new(
				rng.random_range(group_range),
				rng.random_range(number_range),
			))
		}
	}

	pub fn new(group: u8, number: u32) -> Self {
		Self { group, number }
	}

	pub fn group(&self) -> u8 {
		self.group
	}

	pub fn number(&self) -> u32 {
		self.number
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
	fn playground() {
		const MASK: u32 = (0x00_FF_FF_FF);
		let mut hoge = 0u32;
		let piyo = 19u8;

		hoge |= piyo as u32;
		hoge <<= 24;
		println!("{hoge:0x}");

		hoge |= 114_514;
		println!("{hoge:0x}");

		println!("{:0x}", (hoge & MASK));
	}

	fn encode(group: u8, number: u32) -> u32 {
		(group as u32) << 24 | (number & 0x00_FF_FF_FF)
	}

	fn decode(scr: u32) -> (u8, u32) {
		let group = (scr >> 24) as u8;
		let number = scr & 0x00_FF_FF_FF;
		(group, number)
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

		#[expect(
			clippy::reversed_empty_ranges,
			reason = "This is a test for invalid ranges, so the reversed range is intentional."
		)]
		let fixture = Ticket::try_from_rng(100..0, 0..100, &mut mock);

		assert_matches!(fixture, Err(Error::InvalidGroupRange));

		assert_matches!(
			Ticket::try_from_rng(1..100, 0..0, &mut mock),
			Err(Error::InvalidNumberRange)
		);

		#[expect(
			clippy::reversed_empty_ranges,
			reason = "This is a test for invalid ranges, so the reversed range is intentional."
		)]
		let fixture = Ticket::try_from_rng(1..100, 1000..100, &mut mock);

		assert_matches!(fixture, Err(Error::InvalidNumberRange));
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

		assert_eq!(fixture.number(), 114514);
	}

	#[test]
	fn group() {
		let fixture = Ticket {
			number: 114514,
			group: 19,
		};

		assert_eq!(fixture.group(), 19);
	}
}
