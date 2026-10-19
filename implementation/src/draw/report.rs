#![expect(unused_variables, reason = "Under construction")]
#![expect(dead_code, reason = "under construction")]
#![expect(unfulfilled_lint_expectations, reason = "Under construction")]
use super::{Group, Number};
pub struct Record {
	rank: u8,
	payout: u64,
	group: Group,
	number: Number,
}

impl Record {
	pub fn new(rank: u8, payout: u64, group: Group, number: Number) -> Self {
		Self {
			rank,
			payout,
			group,
			number,
		}
	}

	pub fn rank(&self) -> u8 {
		self.rank
	}

	pub fn payout(&self) -> u64 {
		self.payout
	}

	pub fn group(&self) -> &Group {
		&self.group
	}

	pub fn number(&self) -> &Number {
		&self.number
	}
}

pub struct Report(Vec<Record>);

impl Report {
	pub fn new(records: Vec<Record>) -> Self {
		todo!()
	}

	pub fn verify(&self, ticket: &crate::Ticket) -> bool {
		todo!()
	}

	pub fn records(&self) -> &[Record] {
		todo!()
	}
}
