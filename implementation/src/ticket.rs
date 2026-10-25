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

const NUMBER_MASK: u32 = 0x00_ff_ff_ff;
const GROUP_OFFSET: usize = 24;

fn encode(group: u8, number: u32) -> u32 {
	(group as u32) << GROUP_OFFSET | (number & NUMBER_MASK)
}

fn decode(scr: u32) -> (u8, u32) {
	let group = (scr >> GROUP_OFFSET) as u8;
	let number = scr & NUMBER_MASK;
	(group, number)
}
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Ticket(u32);

impl Ticket {
	pub fn new(group: u8, number: u32) -> Self {
		Self(encode(group, number))
	}

	pub fn group(&self) -> u8 {
		decode(self.0).0
	}

	pub fn number(&self) -> u32 {
		decode(self.0).1
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
	use std::hash::{DefaultHasher, Hash, Hasher};

	mock! {
		pub Rng {}

		impl TryRng for Rng {
			type Error = Infallible;

			fn try_next_u32(&mut self) -> std::result::Result<u32, Infallible>;
			fn try_next_u64(&mut self) -> std::result::Result<u64, Infallible>;
			fn try_fill_bytes(&mut self, dest: &mut [u8]) -> std::result::Result<(), Infallible>;
		}
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
	fn new() {
		for g in 1u8..10 {
			for n in 1u32..100 {
				let fixture = Ticket::new(g, n);
				let exp = encode(g, n);
				assert_eq!(fixture.0, exp);
			}
		}
	}

	#[test]
	fn number() {
		let fixture = Ticket(encode(19, 114514));

		assert_eq!(fixture.number(), 114514);
	}

	#[test]
	fn group() {
		let fixture = Ticket(encode(19, 114514));
		assert_eq!(fixture.group(), 19);
	}

	#[test]
	fn eq_test() {
		let mut rng_left = Xoshiro256PlusPlus::from_rng(&mut rand::rng());
		let mut rng_right = rng_left.clone();
		let mut rng_other = rng_right.clone();

		for _ in 0..1_000 {
			let left = Ticket(rng_left.next_u32());
			let right = Ticket(rng_right.next_u32());
			let other = Ticket(rng_other.next_u32());

			assert!(left == left);
			assert!(left == right);
			assert!(right == left);
			assert!(right == other);
			assert!(left == other);
		}
	}

	#[test]
	fn hash_test() {
		let mut rng_left = Xoshiro256PlusPlus::from_rng(&mut rand::rng());
		let mut rng_right = rng_left.clone();

		for _ in 0u32..1_000 {
			let mut l_h = DefaultHasher::new();
			let mut r_h = DefaultHasher::new();

			let left = Ticket(rng_left.next_u32());

			left.hash(&mut l_h);
			Ticket(rng_right.next_u32()).hash(&mut r_h);

			assert_eq!(l_h.finish(), r_h.finish());
		}
	}

	#[test]
	fn clone() {
		let fixture = Ticket::new(42, 114514);
		let mut cloned = fixture.clone();

		assert_eq!(fixture.0, cloned.0);
		cloned.0 = 1_145_141_919;
		assert_ne!(fixture.0, cloned.0);
	}
}
