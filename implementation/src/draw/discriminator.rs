use crate::{Group, Number, PrizeCategory};
use rust_decimal::Decimal;

pub struct Discriminator(Vec<(PrizeCategory, Number, Group, Decimal)>);

impl Discriminator {
	pub fn new(records: Vec<(PrizeCategory, Number, Group, Decimal)>) -> Self {
		Self(records)
	}

	pub fn check(&self, ticket: &crate::Ticket) -> super::outcome::Outcome {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use crate::{Number, PrizeCategory};

	#[test]
	fn new() {
		todo!()
	}
}
