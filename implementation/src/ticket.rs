use rand::Rng;
use std::range::Range;

pub enum Error {
	InvalidGroupRange,
	InvalidNumberRange,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Ticket {
	group: u8,
	number: u32,
}

impl Ticket {
	pub fn try_new(
		group_range: Range<u8>,
		number_range: Range<u32>,
		rng: &mut impl Rng,
	) -> Result<Self> {
		todo!()
	}

	pub fn group(&self) -> u8 {
		todo!()
	}

	pub fn number(&self) -> u32 {
		todo!()
	}
}
