use super::{Group, Number};
pub struct Record {
	rank: u8,
	payout: u64,
	group: Group,
	number: Number,
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
