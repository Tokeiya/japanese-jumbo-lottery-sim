#![expect(unused, reason = "Under construction")]
#![expect(dead_code, reason = "under construction")]

use crate::Ticket;

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum GroupKind {
	AllDigits(u8),
	Last1Digits(u8),
	None,
}

pub struct Group(GroupKind);

impl Group {
	pub fn all_digits(value: u8) -> Self {
		Self(GroupKind::AllDigits(value))
	}

	pub fn last_one_digits(value: u8) -> Self {
		//Probably panic if value is not in 0..=9
		if value > 9 {
			panic!("Value must be in 0..=9");
		}
		Self(GroupKind::Last1Digits(value))
	}

	pub fn verify(&self, ticket: &Ticket) -> bool {
		todo!()
	}
}

#[cfg(test)]
mod tests {

	use super::*;
	use std::assert_matches;
}
