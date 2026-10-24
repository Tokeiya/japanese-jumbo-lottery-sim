#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
	AllDigits,
	LastFourDigits,
	LastThreeDigits,
	LastTwoDigits,
	LastOneDigit,
}

pub struct Number(Kind, u32);

impl Number {
	pub fn all_digits(value: u32) -> Self {
		Self(Kind::AllDigits, value)
	}

	pub fn last_four_digits(value: u32) -> Self {
		//Probably panic if value is not in 0..=9999
		if value >= 10_000 {
			panic!("Value out of range for last four digits");
		}
		Self(Kind::LastFourDigits, value)
	}

	pub fn last_three_digits(value: u32) -> Self {
		//Probably panic if value is not in 0..=999
		if value >= 1_000 {
			panic!("Value out of range for last three digits");
		}
		Self(Kind::LastThreeDigits, value)
	}

	pub fn last_two_digits(value: u32) -> Self {
		//Probably panic if value is not in 0..=99
		if value >= 100 {
			panic!("Value out of range for last two digits");
		}
		Self(Kind::LastTwoDigits, value)
	}

	pub fn last_one_digit(value: u32) -> Self {
		//Probably panic if value is not in 0..=9
		if value >= 10 {
			panic!("Value out of range for last one digit");
		}
		Self(Kind::LastOneDigit, value)
	}

	pub fn verify(&self, ticket: &crate::Ticket) -> bool {
		match self.0 {
			Kind::AllDigits => self.1 == ticket.number(),
			Kind::LastFourDigits => self.1 == ticket.number() % 10_000,
			Kind::LastThreeDigits => self.1 == ticket.number() % 1_000,
			Kind::LastTwoDigits => self.1 == ticket.number() % 100,
			Kind::LastOneDigit => self.1 == ticket.number() % 10,
		}
	}

	pub fn value(&self) -> u32 {
		self.1
	}

	pub fn kind(&self) -> Kind {
		self.0
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::ticket::Ticket;
	use rand::distr::uniform::Error;
	use rand::distr::uniform::SampleRange;
	use rand::{Rng, RngExt};
	struct NumberRange;

	impl SampleRange<u32> for NumberRange {
		fn sample_single<R: Rng + ?Sized>(self, rng: &mut R) -> Result<u32, Error> {
			Ok(rng.random_range(100000u32..200000))
		}

		fn is_empty(&self) -> bool {
			false
		}
	}
	#[test]
	fn all_digits() {
		let mut rng = rand::rng();

		for (exp, grp) in
			(0..200).map(|_| (rng.random_range(NumberRange), rng.random_range(1u8..=100)))
		{
			let fixture = Number::all_digits(exp);
			let ticket = Ticket::new(grp, exp);
			assert!(fixture.verify(&ticket));
			assert!(!fixture.verify(&Ticket::new(grp, exp.wrapping_add(1))));

			assert_eq!(fixture.value(), exp);
			assert_eq!(fixture.kind(), Kind::AllDigits);
		}
	}

	#[test]
	fn last_four_digits() {
		let mut rng = rand::rng();

		for _ in 0..200 {
			let exp = rng.random_range(0u32..10_000);
			let fixture = Number::last_four_digits(exp);
			let act = rng.random_range(0u32..10u32) * 10_000 + exp;
			assert!(fixture.verify(&Ticket::new(1, act)));
			assert!(!fixture.verify(&Ticket::new(1, exp.wrapping_sub(1))));

			assert_eq!(fixture.value(), exp);
			assert_eq!(fixture.kind(), Kind::LastFourDigits);
		}
	}

	#[test]
	#[should_panic]
	fn last_four_digits_invalid() {
		_ = Number::last_four_digits(10_000);
	}

	#[test]
	fn last_three_didits() {
		let mut rng = rand::rng();

		for _ in 0..200 {
			let exp = rng.random_range(0u32..1_000);
			let fixture = Number::last_three_digits(exp);
			let act = rng.random_range(0u32..100u32) * 1_000 + exp;
			assert!(fixture.verify(&Ticket::new(1, act)));
			assert!(!fixture.verify(&Ticket::new(1, exp.wrapping_sub(1))));

			assert_eq!(fixture.value(), exp);
			assert_eq!(fixture.kind(), Kind::LastThreeDigits);
		}
	}

	#[test]
	#[should_panic]
	fn last_three_digits_invalid() {
		_ = Number::last_three_digits(1_000);
	}

	#[test]
	fn last_two_digits() {
		let mut rng = rand::rng();

		for _ in 0..200 {
			let exp = rng.random_range(0u32..100);
			let fixture = Number::last_two_digits(exp);
			let act = rng.random_range(0u32..1_000u32) * 100 + exp;
			assert!(fixture.verify(&Ticket::new(1, act)));
			assert!(!fixture.verify(&Ticket::new(1, exp.wrapping_sub(1))));
			assert_eq!(fixture.value(), exp);
			assert_eq!(fixture.kind(), Kind::LastTwoDigits);
		}
	}

	#[test]
	#[should_panic]
	fn last_two_digits_invalid() {
		_ = Number::last_two_digits(100);
	}

	#[test]
	fn last_one_digit() {
		let mut rng = rand::rng();

		for _ in 0..200 {
			let exp = rng.random_range(0u32..10);
			let fixture = Number::last_one_digit(exp);
			let act = rng.random_range(0u32..10_000u32) * 10 + exp;
			assert!(fixture.verify(&Ticket::new(1, act)));
			assert!(!fixture.verify(&Ticket::new(1, exp.wrapping_sub(1))));

			assert_eq!(fixture.value(), exp);
			assert_eq!(fixture.kind(), Kind::LastOneDigit);
		}
	}

	#[test]
	#[should_panic]
	fn last_one_digit_invalid() {
		_ = Number::last_one_digit(10);
	}
}
