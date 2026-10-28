use crate::Ticket;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
	AllDigits,
	LastOneDigit,
	None,
}

pub struct Group(Kind, u8);

impl Group {
	pub fn all_digits(value: u8) -> Self {
		Self(Kind::AllDigits, value)
	}

	pub fn last_one_digit(value: u8) -> Self {
		//Probably panic if value is not in 0..=9
		if value > 9 {
			panic!("Value must be in 0..=9");
		}
		Self(Kind::LastOneDigit, value)
	}

	pub fn none() -> Self {
		Self(Kind::None, 0)
	}

	pub fn verify(&self, ticket: &Ticket) -> bool {
		match self.0 {
			Kind::AllDigits => ticket.group() == self.1,
			Kind::LastOneDigit => ticket.group() % 10 == self.1,
			Kind::None => true,
		}
	}

	pub fn kind(&self) -> Kind {
		self.0
	}

	pub fn value(&self) -> Option<u8> {
		match self.0 {
			Kind::AllDigits => Some(self.1),
			Kind::LastOneDigit => Some(self.1),
			Kind::None => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::assert_matches;

	#[test]
	fn all_digits() {
		let fixture = Group::all_digits(42);
		assert_matches!(fixture.0, Kind::AllDigits);
		assert_eq!(fixture.1, 42);
	}

	#[test]
	fn last_one_digits() {
		let fixture = Group::last_one_digit(5);
		assert_matches!(fixture.0, Kind::LastOneDigit);
		assert_eq!(fixture.1, 5);
	}

	#[should_panic]
	#[test]
	fn invalid_last_one_digits() {
		_ = Group::last_one_digit(10);
	}

	#[test]
	fn none() {
		let fixture = Group::none();
		assert_matches!(fixture.0, Kind::None);
		assert_eq!(fixture.1, 0);
	}

	#[test]
	fn kind() {
		let fixture = Group(Kind::AllDigits, 42);
		assert_matches!(fixture.kind(), Kind::AllDigits);

		let fixture = Group(Kind::LastOneDigit, 2);
		assert_matches!(fixture.kind(), Kind::LastOneDigit);

		let fixture = Group(Kind::None, 0);
		assert_matches!(fixture.kind(), Kind::None);
	}

	#[test]
	fn value() {
		let fixture = Group(Kind::AllDigits, 42);
		assert_matches!(fixture.value(),Some(i) if i==42);

		let fixture = Group(Kind::LastOneDigit, 4);
		assert_matches!(fixture.value(),Some(i) if i==4);

		assert_matches!(Group(Kind::None, 0).value(), None);
	}

	#[test]
	fn verify() {
		let fixture = Group(Kind::AllDigits, 42);

		for i in 1u8..=100 {
			let ticket = Ticket::new(10, i, 114514);
			if i == 42 {
				assert!(fixture.verify(&ticket));
			} else {
				assert!(!fixture.verify(&ticket));
			}
		}

		let fixture = Group(Kind::LastOneDigit, 1);

		for i in 1u8..=100 {
			let ticket = Ticket::new(10, i, 114514);
			if i % 10 == 1 {
				assert!(fixture.verify(&ticket));
			} else {
				assert!(!fixture.verify(&ticket));
			}
		}

		let fixture = Group(Kind::None, 0);

		for i in 1u8..=100 {
			let ticket = Ticket::new(10, i, 114514);

			assert!(fixture.verify(&ticket));
		}
	}
}
