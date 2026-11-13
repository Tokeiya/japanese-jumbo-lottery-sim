use crate::PrizeCategory;
use crate::draw::group::Kind as GroupKind;
use crate::draw::number::Kind as NumberKind;
use rust_decimal::Decimal;

pub struct Record {
	kind: PrizeCategory,
	number_kind: NumberKind,
	group_kind: GroupKind,
	prize: Decimal,
}

impl Record {
	pub fn new(
		kind: PrizeCategory,
		number_kind: NumberKind,
		group_kind: GroupKind,
		prize: Decimal,
	) -> Self {
		Self {
			kind,
			number_kind,
			group_kind,
			prize,
		}
	}

	pub fn number_kind(&self) -> NumberKind {
		self.number_kind
	}

	pub fn group_kind(&self) -> GroupKind {
		self.group_kind
	}

	pub fn prize(&self) -> Decimal {
		self.prize
	}

	pub fn kind(&self) -> PrizeCategory {
		self.kind
	}
}

pub struct Spec {
	name: String,
	group_size: u8,
	unit_price: Decimal,
	prize: Vec<Record>,
}

impl Spec {
	pub fn new(name: String, group_size: u8, unit_price: Decimal, prize: Vec<Record>) -> Self {
		Self {
			name,
			group_size,
			unit_price,
			prize,
		}
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn prize(&self) -> &[Record] {
		&self.prize
	}

	pub fn group_size(&self) -> u8 {
		self.group_size
	}

	pub fn unit_price(&self) -> Decimal {
		self.unit_price
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	mod spec {
		use super::*;

		fn assert(actual: &Record, expected: &Record) {
			assert_eq!(actual.kind, expected.kind);
			assert_eq!(actual.number_kind, expected.number_kind);
			assert_eq!(actual.group_kind, expected.group_kind);
			assert_eq!(actual.prize, expected.prize);
		}

		#[test]
		fn new() {
			let v = vec![
				Record::new(
					PrizeCategory::First,
					NumberKind::AllDigits,
					GroupKind::AllDigits,
					500.into(),
				),
				Record::new(
					PrizeCategory::FirstAdjacent,
					NumberKind::AllDigits,
					GroupKind::AllDigits,
					5000.into(),
				),
			];

			let fixture = Spec::new("Name".into(), 100, 200.into(), v);

			assert_eq!(fixture.name, "Name");
			assert_eq!(fixture.prize.len(), 2);

			assert_eq!(fixture.group_size, 100);
			assert_eq!(fixture.unit_price, 200.into());

			assert(
				&fixture.prize[0],
				&Record::new(
					PrizeCategory::First,
					NumberKind::AllDigits,
					GroupKind::AllDigits,
					500.into(),
				),
			);

			assert(
				&fixture.prize[1],
				&Record::new(
					PrizeCategory::FirstAdjacent,
					NumberKind::AllDigits,
					GroupKind::AllDigits,
					5000.into(),
				),
			);
		}

		#[test]
		fn name() {
			let fixture = Spec::new("Name".into(), 100, 200.into(), vec![]);
			assert_eq!(fixture.name(), "Name");
		}

		#[test]
		fn prize() {
			let fixture = Spec::new(
				"Name".into(),
				100,
				200.into(),
				vec![Record::new(
					PrizeCategory::First,
					NumberKind::AllDigits,
					GroupKind::AllDigits,
					500.into(),
				)],
			);
			assert_eq!(fixture.prize().len(), 1);
			assert(&fixture.prize[0], &fixture.prize()[0]);
		}

		#[test]
		fn group_size() {
			let fixture = Spec::new("Name".into(), 100, 200.into(), vec![]);
			assert_eq!(fixture.group_size(), 100);
		}

		#[test]
		fn unit_price() {
			let fixture = Spec::new("Name".into(), 100, 200.into(), vec![]);
			assert_eq!(fixture.unit_price(), 200.into());
		}
	}
	mod record {
		use super::*;

		fn generate() -> Record {
			Record {
				kind: PrizeCategory::Fifth,
				number_kind: NumberKind::AllDigits,
				group_kind: GroupKind::None,
				prize: 200.into(),
			}
		}
		#[test]
		fn new() {
			let fixture = Record::new(
				PrizeCategory::Fifth,
				NumberKind::AllDigits,
				GroupKind::None,
				100.into(),
			);

			assert_eq!(fixture.kind, PrizeCategory::Fifth);
			assert_eq!(fixture.number_kind, NumberKind::AllDigits);
			assert_eq!(fixture.group_kind, GroupKind::None);
			assert_eq!(fixture.prize, 100.into());
		}

		#[test]
		fn number_kind() {
			let fixture = generate();
			assert_eq!(fixture.number_kind(), fixture.number_kind);
		}

		#[test]
		fn group_test() {
			let fixture = generate();
			assert_eq!(fixture.group_kind(), fixture.group_kind);
		}

		#[test]
		fn prize() {
			let fixture = generate();
			assert_eq!(fixture.prize(), fixture.prize);
		}
	}
}
