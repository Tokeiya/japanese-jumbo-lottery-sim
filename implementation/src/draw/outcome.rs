use crate::ticket::Ticket;

#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Record {
	rank: u8,
	payout: u64,
}

impl Record {
	fn new(rank: u8, payout: u64) -> Self {
		Self { rank, payout }
	}

	pub fn rank(&self) -> u8 {
		self.rank
	}

	pub fn payout(&self) -> u64 {
		self.payout
	}
}

pub struct Outcome(Ticket, Vec<Record>);

impl Outcome {
	pub fn new(ticket: Ticket) -> Self {
		Self(ticket, Vec::new())
	}

	pub(super) fn add_record(&mut self, rank: u8, payout: u64) {
		self.1.push(Record::new(rank, payout));
	}

	pub fn records(&self) -> &[Record] {
		&self.1
	}
	pub fn ticket(&self) -> Ticket {
		self.0.clone()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use rand::{RngExt, rng};

	mod outcome {
		use super::*;

		#[test]
		fn new() {
			let mut rng = rng();
			let ticket = Ticket::new(
				rng.random_range(0u8..100),
				rng.random_range(100_000..200_000),
			);

			let outcome = Outcome::new(ticket.clone());

			assert_eq!(outcome.ticket(), ticket);
			assert_eq!(outcome.1.len(), 0);
		}

		#[test]
		fn records() {
			let ticket = Ticket::new(1, 100_000);
			let mut fixture = Outcome::new(ticket);

			fixture.1.push(Record::new(1, 100_000));
			fixture.1.push(Record::new(2, 20000));
			fixture.1.push(Record::new(3, 3000));

			assert_eq!(fixture.records().len(), 3);

			for (act, exp) in fixture.records().iter().zip(
				vec![
					Record::new(1, 100_000),
					Record::new(2, 20000),
					Record::new(3, 3000),
				]
				.iter(),
			) {
				assert_eq!(act.rank, exp.rank);
				assert_eq!(act.payout, exp.payout);
			}
		}

		#[test]
		fn add_record() {
			let mut fixture = Outcome::new(Ticket::new(1, 100_000));
			let mut rng = rng();

			let mut expected = Vec::new();

			for r in 1u8..=4 {
				let payout = rng.random_range(10_000..=1_000_000);
				let record = Record::new(r, payout);
				fixture.add_record(r, payout);
				expected.push(record);
			}

			assert_eq!(fixture.1.len(), 4);

			for exp in expected.iter() {
				assert!(fixture.1.contains(exp))
			}
		}
	}

	mod record {
		use super::*;
		#[test]
		fn new() {
			let record = Record::new(1, 100);
			assert_eq!(record.rank, 1);
			assert_eq!(record.payout, 100);
		}

		#[test]
		fn rank() {
			let record = Record::new(1, 100);
			assert_eq!(record.rank(), 1);
		}

		#[test]
		fn payout() {
			let record = Record::new(1, 100);
			assert_eq!(record.payout(), 100);
		}
	}
}
