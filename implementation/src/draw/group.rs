use crate::Ticket;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupKind {
	AllDigits,
	LastOneDigit,
	None,
}

pub struct Group(GroupKind, u8);

impl Group {
	pub fn all_digits(value: u8) -> Self {
		Self(GroupKind::AllDigits, value)
	}

	pub fn last_one_digits(value: u8) -> Self {
		//Probably panic if value is not in 0..=9
		if value > 9 {
			panic!("Value must be in 0..=9");
		}
		Self(GroupKind::LastOneDigit, value)
	}

	pub fn none() -> Self {
		Self(GroupKind::None, 0)
	}

	pub fn verify(&self, ticket: &Ticket) -> bool {
		match self.0 {
			GroupKind::AllDigits => ticket.group() == self.1,
			GroupKind::LastOneDigit => ticket.group() % 10 == self.1,
			GroupKind::None => true,
		}
	}

	pub fn kind(&self) -> GroupKind {
		self.0
	}

	pub fn value(&self) -> Option<u8> {
		match self.0 {
			GroupKind::AllDigits => Some(self.1),
			GroupKind::LastOneDigit => Some(self.1),
			GroupKind::None => None,
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
		assert_matches!(fixture.0, GroupKind::AllDigits);
		assert_eq!(fixture.1, 42);
	}

	#[test]
	fn last_one_digits() {
		let fixture = Group::last_one_digits(5);
		assert_matches!(fixture.0, GroupKind::LastOneDigit);
		assert_eq!(fixture.1, 5);
	}

	#[should_panic]
	#[test]
	fn invalid_last_one_digits() {
		_ = Group::last_one_digits(10);
	}

	#[test]
	fn none() {
		let fixture = Group::none();
		assert_matches!(fixture.0, GroupKind::None);
		assert_eq!(fixture.1, 0);
	}

	#[test]
	fn kind() {
		let fixture = Group(GroupKind::AllDigits, 42);
		assert_matches!(fixture.kind(), GroupKind::AllDigits);

		let fixture = Group(GroupKind::LastOneDigit, 2);
		assert_matches!(fixture.kind(), GroupKind::LastOneDigit);

		let fixture = Group(GroupKind::None, 0);
		assert_matches!(fixture.kind(), GroupKind::None);
	}

	#[test]
	fn value() {
		let fixture = Group(GroupKind::AllDigits, 42);
		assert_matches!(fixture.value(),Some(i) if i==42);

		let fixture = Group(GroupKind::LastOneDigit, 4);
		assert_matches!(fixture.value(),Some(i) if i==4);

		assert_matches!(Group(GroupKind::None, 0).value(), None);
	}

	#[test]
	fn verify() {
		let fixture = Group(GroupKind::AllDigits, 42);

		for i in 1u8..=100 {
			let ticket = Ticket::new(i, 114514);
			if i == 42 {
				assert!(fixture.verify(&ticket));
			} else {
				assert!(!fixture.verify(&ticket));
			}
		}

		let fixture = Group(GroupKind::LastOneDigit, 1);

		for i in 1u8..=100 {
			let ticket = Ticket::new(i, 114514);
			if i % 10 == 1 {
				assert!(fixture.verify(&ticket));
			} else {
				assert!(!fixture.verify(&ticket));
			}
		}

		let fixture = Group(GroupKind::None, 0);

		for i in 1u8..=100 {
			let ticket = Ticket::new(i, 114514);

			assert!(fixture.verify(&ticket));
		}
	}
}
