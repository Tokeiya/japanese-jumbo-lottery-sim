#![expect(unused_variables, reason = "Under construction")]
#![expect(dead_code, reason = "under construction")]
#![expect(unfulfilled_lint_expectations, reason = "Under construction")]

pub struct Record {
	rank: u8,
	payout: u64,
}

pub struct Outcome(Vec<Record>);

impl Outcome {
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
