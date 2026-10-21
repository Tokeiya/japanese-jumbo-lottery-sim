use crate::Ticket;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Group {
	AllDigits(u8),
	Last1Digits(u8),
	None,
}

impl Group {
	pub fn all_digits(value: u8) -> Self {
		Self::AllDigits(value)
	}

	pub fn last_one_digits(value: u8) -> Self {
		//Probably panic if value is not in 0..=9
		if value > 9 {
			panic!("Value must be in 0..=9");
		}
		Self::Last1Digits(value)
	}

	pub fn verify(&self, ticket: &Ticket) -> bool {
		match self {
			Group::AllDigits(x) => ticket.group() == *x,
			Group::Last1Digits(x) => ticket.group() % 10 == *x,
			Group::None => true,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::assert_matches;
	#[test]
	fn test_all_digits() {
		let group = Group::all_digits(42);
		assert_matches!(group, Group::AllDigits(x) if x==42);
	}

	#[test]
	fn test_last_one_digits() {
		let group = Group::last_one_digits(3);
		assert_matches!(group, Group::Last1Digits(x) if x==3);
	}

	#[should_panic]
	#[test]
	fn test_last_one_digits_panic() {
		_ = Group::last_one_digits(10);
	}

	#[test]
	fn all_digits_verify() {
		for i in 1..=100u8 {
			let fixture = Group::AllDigits(i);

			for t in 1..=100u8 {
				let ticket = Ticket::new(t, 114514);
				if t == i {
					assert!(fixture.verify(&ticket));
				} else {
					assert!(!fixture.verify(&ticket));
				}
			}
		}
	}

	#[test]
	fn one_digits_verify() {
		for i in 0..=9u8 {
			let fixture = Group::Last1Digits(i);

			for g in 0..=100u8 {
				if g % 10 == i {
					assert!(fixture.verify(&Ticket::new(g, 114514)));
				} else {
					assert!(!fixture.verify(&Ticket::new(g, 114514)));
				}
			}
		}
	}

	#[test]
	fn none_verify() {
		let fixture = Group::None;
		for t in 1..=100u8 {
			let ticket = Ticket::new(t, 114514);
			assert!(fixture.verify(&ticket));
		}
	}
}
