#![expect(unused_variables, reason = "Under construction")]
#![expect(unfulfilled_lint_expectations, reason = "Under construction")]

pub enum Number {
	AllDigits(u32),
	Last4Digits(u16),
	Last3Digits(u16),
	Last2Digits(u8),
	Last1Digit(u8),
}

impl Number {
	pub fn all_digits(value: u32) -> Self {
		todo!()
	}

	pub fn last_four_digits(value: u16) -> Self {
		//Probably panic if value is not in 0..=9999
		todo!()
	}

	pub fn last_three_digits(value: u16) -> Self {
		//Probably panic if value is not in 0..=999
		todo!()
	}

	pub fn last_two_digits(value: u8) -> Self {
		//Probably panic if value is not in 0..=99
		todo!()
	}

	pub fn last_one_digit(value: u8) -> Self {
		//Probably panic if value is not in 0..=9
		todo!()
	}

	pub fn verify(&self, ticket: &crate::Ticket) -> bool {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
}
