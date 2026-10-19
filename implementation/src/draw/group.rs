#![expect(unused_variables, reason = "Under construction")]
#![expect(unfulfilled_lint_expectations, reason = "Under construction")]

use crate::Ticket;

#[derive(Debug)]
pub enum Group {
	AllDigits(u8),
	Last1Digits(u8),
}

impl Group {
	pub fn all_digits(value: u8) -> Self {
		todo!()
	}

	pub fn last_one_digits(value: u8) -> Self {
		//Probably panic if value is not in 0..=9
		todo!()
	}

	pub fn verify(&self, ticket: &Ticket) -> bool {
		todo!()
	}
}
