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
		Self(records)
	}

	pub fn verify(&self, ticket: &crate::Ticket) -> bool {
		todo!()
	}

	pub fn records(&self) -> &[Record] {
		self.0.as_slice()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::assert_matches;

	mod report {
		use super::*;
		#[test]
		fn new() {
			let records = vec![
				Record::new(1, 100, Group::AllDigits(42), Number::all_digits(114514)),
				Record::new(
					2,
					200,
					Group::Last1Digits(4),
					Number::last_four_digits(4515),
				),
			];
			let report = Report::new(records);
			assert_eq!(report.records().len(), 2);

			for (a, e) in report.0.iter().zip(vec![
				Record::new(1, 100, Group::AllDigits(42), Number::all_digits(114514)),
				Record::new(
					2,
					200,
					Group::Last1Digits(4),
					Number::last_four_digits(4515),
				),
			]) {
				assert_eq!(a.rank(), e.rank());
				assert_eq!(a.payout(), e.payout());
				assert_eq!(a.group(), e.group());
				assert_eq!(a.number(), e.number());
			}
		}

		#[test]
		fn rank() {
			let records = vec![
				Record::new(1, 100, Group::AllDigits(42), Number::all_digits(114514)),
				Record::new(
					2,
					200,
					Group::Last1Digits(4),
					Number::last_four_digits(4515),
				),
			];
			let report = Report::new(records);
			assert_eq!(report.records()[0].rank(), 1);
			assert_eq!(report.records()[1].rank(), 2);
		}
	}

	mod record {
		use super::*;
		#[test]
		fn new() {
			let record = Record::new(1, 100, Group::AllDigits(42), Number::all_digits(114514));
			assert_eq!(record.rank, 1);
			assert_eq!(record.payout, 100);

			assert_eq!(record.group, Group::AllDigits(42));
			assert_eq!(record.number, Number::all_digits(114514));
		}

		#[test]
		fn rank() {
			let record = Record::new(1, 100, Group::AllDigits(42), Number::all_digits(114514));
			assert_eq!(record.rank(), 1);
		}

		#[test]
		fn payout() {
			let record = Record::new(1, 100, Group::AllDigits(42), Number::all_digits(114514));
			assert_eq!(record.payout(), 100);
		}

		#[test]
		fn group() {
			let record = Record::new(1, 100, Group::AllDigits(42), Number::all_digits(114514));
			assert_matches!(record.group(), Group::AllDigits(x) if *x==42);
		}

		#[test]
		fn number() {
			let record = Record::new(1, 100, Group::AllDigits(42), Number::all_digits(114514));
			assert_matches!(record.number(), Number::AllDigits(x) if *x==114514);
		}
	}
}
