use super::{Group, Number, Payout};
use crate::{PrizeCategory, Ticket};
use rust_decimal::Decimal;

pub struct Record {
	rank: PrizeCategory,
	payout: Decimal,
	group: Group,
	number: Number,
}

impl Record {
	pub fn new(rank: PrizeCategory, payout: Decimal, group: Group, number: Number) -> Self {
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

	pub fn payout(&self) -> Decimal {
		self.payout
	}

	pub fn group(&self) -> &Group {
		&self.group
	}

	pub fn number(&self) -> &Number {
		&self.number
	}

	pub fn verify(&self, ticket: &Ticket) -> bool {
		#[cfg(test)]
		{
			if self.rank == PrizeCategory::Second {
				println!(
					"verify: ticket group {}, number {}",
					ticket.group(),
					ticket.number()
				);

				println!("group:{:?}", self.number);

				println!("num:{}", self.number.verify(ticket));
			}
		}

		self.group.verify(ticket) && self.number.verify(ticket)
	}
}

pub struct Outcome(Vec<Record>);

impl Outcome {
	pub fn new(records: Vec<Record>) -> Self {
		Self(records)
	}

	pub fn verify(&self, ticket: &crate::Ticket) -> Payout {
		let mut records: Vec<super::payout::Record> = Vec::new();
		let mut flg = false;

		for prize in self.0.iter() {
			if prize.verify(ticket) {
				if prize.rank == PrizeCategory::First {
					flg = true;
				};

				if (prize.rank == PrizeCategory::FirstDifferentGroup && !flg)
					|| prize.rank != PrizeCategory::FirstDifferentGroup
				{
					records.push(super::payout::Record::new(prize.rank, prize.payout));
				}
			}
		}

		Payout::new(ticket.clone(), records)
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
					100.into(),
					Group::all_digits(42),
					Number::all_digits(114514),
				),
				Record::new(
					PrizeCategory::Second,
					200.into(),
					Group::last_one_digit(4),
					Number::last_four_digits(4515),
				),
			]
		}

		#[test]
		fn new() {
			let records = sample_records();
			let report = Outcome::new(records);
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
			let report = Outcome::new(records);

			assert_matches!(report.records()[0].rank(), PrizeCategory::First);
			assert_matches!(report.records()[1].rank(), PrizeCategory::Second);
		}

		#[test]
		fn verify() {
			let mut records = sample_records();
			records.push(Record::new(
				PrizeCategory::Third,
				300.into(),
				Group::last_one_digit(2),
				Number::last_four_digits(4514),
			));
			let report = Outcome::new(records);

			let ticket = Ticket::new(10, 42, 114514);
			let outcome = report.verify(&ticket);

			assert_eq!(outcome.records().len(), 2);
			assert_matches!(outcome.records()[0].rank(), PrizeCategory::First);
			assert_matches!(outcome.records()[1].rank(), PrizeCategory::Third);

			assert_eq!(outcome.records()[0].payout(), 100.into());
			assert_eq!(outcome.records()[1].payout(), 300.into());
		}

		#[test]
		fn first_adjacent_verify() {
			let records = vec![
				Record::new(
					PrizeCategory::First,
					1_000.into(),
					Group::all_digits(42),
					Number::all_digits(114514),
				),
				Record::new(
					PrizeCategory::FirstAdjacent,
					100.into(),
					Group::none(),
					Number::all_digits(114514),
				),
				Record::new(
					PrizeCategory::Second,
					200.into(),
					Group::last_one_digit(2),
					Number::last_four_digits(4514),
				),
				Record::new(
					PrizeCategory::Third,
					30.into(),
					Group::last_one_digit(1),
					Number::last_four_digits(4514),
				),
			];

			let report = Outcome::new(records);
			let ticket = Ticket::new(10, 42, 114514);
			let actual = report.verify(&ticket);

			assert_eq!(actual.records().len(), 2);

			assert_matches!(actual.records()[0].rank(), PrizeCategory::First);
			assert_eq!(actual.records()[0].payout(), 1_000.into());

			assert_matches!(actual.records()[1].rank(), PrizeCategory::Second);
			assert_eq!(actual.records()[1].payout(), 200.into());

			let ticket = Ticket::new(10, 41, 114514);

			let actual = report.verify(&ticket);
			assert_eq!(actual.records().len(), 2);

			assert_matches!(actual.records()[0].rank(), PrizeCategory::FirstAdjacent);
			assert_eq!(actual.records()[0].payout(), 100.into());

			assert_matches!(actual.records()[1].rank(), PrizeCategory::Third);
			assert_eq!(actual.records()[1].payout(), 30.into());
		}
	}

	mod record {
		use super::*;
		use crate::draw::group::Kind as GroupKind;
		use crate::draw::number::Kind as NumberKind;

		#[test]
		fn new() {
			let record = Record::new(
				PrizeCategory::First,
				100.into(),
				Group::all_digits(42),
				Number::all_digits(114514),
			);
			assert_matches!(record.rank(), PrizeCategory::First);
			assert_eq!(record.payout, 100.into());

			assert_eq!(record.group.value(), Some(42));
			assert_matches!(record.group.kind(), GroupKind::AllDigits);

			assert_matches!(record.number.kind(), NumberKind::AllDigits);
			assert_eq!(record.number.value(), 114514);
		}

		#[test]
		fn verify() {
			let record = Record::new(
				PrizeCategory::First,
				100.into(),
				Group::all_digits(42),
				Number::all_digits(114514),
			);
			let ticket = Ticket::new(10, 42, 11_4514);
			let other = Ticket::new(10, 52, 13_4514);

			assert!(record.verify(&ticket));
			assert!(!record.verify(&other));

			let record = Record::new(
				PrizeCategory::First,
				100.into(),
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
				100.into(),
				Group::all_digits(42),
				Number::all_digits(114514),
			);
			assert_matches!(record.rank(), PrizeCategory::First);
		}

		#[test]
		fn payout() {
			let record = Record::new(
				PrizeCategory::First,
				100.into(),
				Group::all_digits(42),
				Number::all_digits(114514),
			);
			assert_eq!(record.payout(), 100.into());
		}

		#[test]
		fn group() {
			let record = Record::new(
				PrizeCategory::First,
				100.into(),
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
				100.into(),
				Group::all_digits(42),
				Number::all_digits(114514),
			);
			assert_matches!(record.number.kind(), NumberKind::AllDigits);
			assert_eq!(record.number.value(), 114514);
		}
	}
}
