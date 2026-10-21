use crate::ticket::Ticket;

pub struct Record {
	rank: u8,
	payout: u64,
}

impl Record {
	pub fn new(rank: u8, payout: u64) -> Self {
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
	pub fn new(ticket: Ticket, records: Vec<Record>) -> Self {
		Self(ticket, records)
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

			let records = vec![Record::new(1, 100_000), Record::new(2, 20000)];
			let outcome = Outcome::new(ticket.clone(), records);

			assert_eq!(outcome.ticket(), ticket);
			assert_eq!(outcome.1.len(), 2);
			let expected = vec![Record::new(1, 100_000), Record::new(2, 20000)];

			for i in 0..expected.len() {
				assert_eq!(outcome.records()[i].payout, expected[i].payout);
				assert_eq!(outcome.records()[i].rank, expected[i].rank);
			}
		}

		#[test]
		fn records() {
			let ticket = Ticket::new(1, 100_000);
			let records = vec![
				Record::new(1, 100_000),
				Record::new(2, 20000),
				Record::new(3, 3000),
			];
			let fixture = Outcome::new(ticket, records);

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
