use super::outcome::{Outcome, PrizeCategory};
use super::{Group, GroupKind, Number, NumberKind, outcome};
use crate::Ticket;
pub struct Record {
	rank: PrizeCategory,
	payout: u64,
	group: Group,
	number: Number,
}

impl Record {
	pub fn new(rank: PrizeCategory, payout: u64, group: Group, number: Number) -> Self {
		Self {
			rank,
			payout,
			group,
			number,
		}
	}

	pub fn rank(&self) -> PrizeCategory {
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

	pub fn verify(&self, ticket: &Ticket) -> bool {
		self.group.verify(ticket) && self.number.verify(ticket)
	}
}

pub struct Report(Vec<Record>);

impl Report {
	pub fn new(records: Vec<Record>) -> Self {
		Self(records)
	}

	pub fn verify(&self, ticket: &crate::Ticket) -> Outcome {
		let mut records: Vec<outcome::Record> = Vec::new();

		for prize in self.0.iter() {
			if prize.verify(ticket) {
				records.push(outcome::Record::new(prize.rank.clone(), prize.payout));
			}
		}

		Outcome::new(ticket.clone(), records)
	}

	pub fn records(&self) -> &[Record] {
		&self.0
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::assert_matches;

	mod report {
		use super::*;
		use crate::Ticket;

		fn sample_records() -> Vec<Record> {
			vec![
				Record::new(
					PrizeCategory::First,
					100,
					Group::all_digits(42),
					Number::all_digits(114514),
				),
				Record::new(
					PrizeCategory::Second,
					200,
					Group::last_one_digit(4),
					Number::last_four_digits(4515),
				),
			]
		}

		#[test]
		fn new() {
			let records = sample_records();
			let report = Report::new(records);
			assert_eq!(report.records().len(), 2);

			for (a, e) in report.0.iter().zip(sample_records()) {
				assert_eq!(a.rank(), e.rank());
				assert_eq!(a.payout(), e.payout());
				assert_eq!(a.group().value(), e.group().value());
				assert_eq!(a.group().kind(), e.group().kind());
				assert_eq!(a.number().value(), e.number().value());
				assert_eq!(a.number().kind(), e.number().kind());
			}
		}

		#[test]
		fn rank() {
			let records = sample_records();
			let report = Report::new(records);

			assert_matches!(report.records()[0].rank(), PrizeCategory::First);
			assert_matches!(report.records()[1].rank(), PrizeCategory::Second);
		}

		#[test]
		fn verify() {
			let mut records = sample_records();
			records.push(Record::new(
				PrizeCategory::Third,
				300,
				Group::last_one_digit(2),
				Number::last_four_digits(4514),
			));
			let report = Report::new(records);

			let ticket = Ticket::new(42, 114514);
			let outcome = report.verify(&ticket);

			assert_eq!(outcome.records().len(), 2);
			assert_matches!(outcome.records()[0].rank(), PrizeCategory::First);
			assert_matches!(outcome.records()[1].rank(), PrizeCategory::Third);

			assert_eq!(outcome.records()[0].payout(), 100);
			assert_eq!(outcome.records()[1].payout(), 300);
		}
	}

	mod record {
		use super::*;

		#[test]
		fn new() {
			let record = Record::new(
				PrizeCategory::First,
				100,
				Group::all_digits(42),
				Number::all_digits(114514),
			);
			assert_matches!(record.rank(), PrizeCategory::First);
			assert_eq!(record.payout, 100);

			assert_eq!(record.group.value(), Some(42));
			assert_matches!(record.group.kind(), GroupKind::AllDigits);

			assert_matches!(record.number.kind(), NumberKind::AllDigits);
			assert_eq!(record.number.value(), 114514);
		}

		#[test]
		fn verify() {
			let record = Record::new(
				PrizeCategory::First,
				100,
				Group::all_digits(42),
				Number::all_digits(114514),
			);
			let ticket = Ticket::new(42, 114514);
			let other = Ticket::new(52, 334514);

			assert!(record.verify(&ticket));
			assert!(!record.verify(&other));

			let record = Record::new(
				PrizeCategory::First,
				100,
				Group::last_one_digit(2),
				Number::last_four_digits(4514),
			);

			assert!(record.verify(&ticket));
			assert!(record.verify(&other));
		}

		#[test]
		fn rank() {
			let record = Record::new(
				PrizeCategory::First,
				100,
				Group::all_digits(42),
				Number::all_digits(114514),
			);
			assert_matches!(record.rank(), PrizeCategory::First);
		}

		#[test]
		fn payout() {
			let record = Record::new(
				PrizeCategory::First,
				100,
				Group::all_digits(42),
				Number::all_digits(114514),
			);
			assert_eq!(record.payout(), 100);
		}

		#[test]
		fn group() {
			let record = Record::new(
				PrizeCategory::First,
				100,
				Group::all_digits(42),
				Number::all_digits(114514),
			);

			assert_matches!(record.group.kind(), GroupKind::AllDigits);
			assert_eq!(record.group.value(), Some(42));
		}

		#[test]
		fn number() {
			let record = Record::new(
				PrizeCategory::First,
				100,
				Group::all_digits(42),
				Number::all_digits(114514),
			);
			assert_matches!(record.number.kind(), NumberKind::AllDigits);
			assert_eq!(record.number.value(), 114514);
		}
	}
}
