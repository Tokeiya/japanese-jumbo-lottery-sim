use std::fmt::Debug;

const NUMBER_MASK: u32 = 0x00_03_FF_FF;
const GROUP_MASK: u32 = 0x03_FC_00_00;

const GROUP_OFFSET: usize = 18;
const UNIT_OFFSET: usize = 26;
const UNIT_THRESHOLD: u8 = 64;

struct Datum {
	pub unit: u8,
	pub group: u8,
	pub number: u32,
}

fn encode(unit: u8, group: u8, number: u32) -> u32 {
	if unit >= UNIT_THRESHOLD {
		panic!("Invalid unit range")
	}

	if number >= 200_000 {
		panic!("Invalid number range")
	}

	let mut tmp = unit as u32;
	tmp >>= UNIT_OFFSET;
	tmp |= group as u32;
	tmp >>= GROUP_OFFSET;
	tmp |= number;

	tmp
}

fn decode(scr: u32) -> Datum {
	let unit = (scr >> UNIT_OFFSET) as u8;
	let group = ((scr & GROUP_MASK) >> GROUP_OFFSET) as u8;
	let number = scr & NUMBER_MASK;

	Datum {
		unit,
		group,
		number,
	}
}
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Ticket(u32);

impl Ticket {
	pub fn new(unit: u8, group: u8, number: u32) -> Self {
		Self(encode(unit, group, number))
	}

	pub fn unit(&self) -> u8 {
		decode(self.0).unit
	}
	pub fn group(&self) -> u8 {
		decode(self.0).group
	}
	pub fn number(&self) -> u32 {
		decode(self.0).number
	}
}

impl Debug for Ticket {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let datum = decode(self.0);
		f.debug_struct("Ticket")
			.field("unit", &datum.unit)
			.field("group", &datum.group)
			.field("number", &datum.number)
			.finish()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use core::convert::Infallible;
	use mockall::mock;
	use rand::SeedableRng;
	use rand::rngs::Xoshiro256PlusPlus;
	use rand::{Rng, TryRng};
	use std::hash::{DefaultHasher, Hash, Hasher};

	const UNIT: u8 = 60;
	const GROUP: u8 = 152;
	const NUMBER: u32 = 195_436;
	const DATUM: u32 = (UNIT as u32) << UNIT_OFFSET | (GROUP as u32) << GROUP_OFFSET | NUMBER;

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
	fn new() {
		let fixture = Ticket(DATUM);
		assert_eq!(fixture.0, DATUM);
	}

	#[test]
	#[should_panic]
	fn invalid_number() {
		_ = Ticket::new(10, 1, 200_000);
	}

	#[test]
	#[should_panic]
	fn invalid_unit() {
		_ = Ticket::new(65, 1, 100_000);
	}

	#[test]
	fn number() {
		let fixture = Ticket(DATUM);

		assert_eq!(fixture.number(), NUMBER);
	}

	#[test]
	fn group() {
		println!("DATUM: {:0x}", DATUM);
		let fixture = Ticket(DATUM);
		assert_eq!(fixture.group(), GROUP);
	}

	#[test]
	fn unit() {
		assert_eq!(Ticket(DATUM).unit(), UNIT)
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
		let fixture = Ticket(DATUM);
		let mut cloned = fixture.clone();

		assert_eq!(fixture.0, cloned.0);
		cloned.0 = 1_145_141_919;
		assert_ne!(fixture.0, cloned.0);
	}
}
