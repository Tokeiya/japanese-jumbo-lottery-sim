use crate::draw::group::Kind as GroupKind;
use crate::draw::number::Kind as NumberKind;
use rust_decimal::Decimal;
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Kind {
	First,
	Adjacent,
	GroupMismatch,
	Second,
	Third,
	Forth,
	Fifth,
	Sixth,
	Seventh,
}

pub struct Record {
	kind: Kind,
	number_kind: NumberKind,
	group_kind: GroupKind,
	prize: Decimal,
}

impl Record {
	pub fn new(kind: Kind, number_kind: NumberKind, group_kind: GroupKind, prize: Decimal) -> Self {
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

	pub fn kind(&self) -> Kind {
		self.kind
	}
}

pub struct Spec {
	name: String,
	prize: Vec<Record>,
}

impl Spec {
	pub fn new(name: String, prize: Vec<Record>) -> Self {
		Self { name, prize }
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn prize(&self) -> &[Record] {
		&self.prize
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
					Kind::First,
					NumberKind::AllDigits,
					GroupKind::AllDigits,
					500.into(),
				),
				Record::new(
					Kind::Adjacent,
					NumberKind::AllDigits,
					GroupKind::AllDigits,
					5000.into(),
				),
			];

			let fixture = Spec::new("Name".into(), v);

			assert_eq!(fixture.name, "Name");
			assert_eq!(fixture.prize.len(), 2);

			assert(
				&fixture.prize[0],
				&Record::new(
					Kind::First,
					NumberKind::AllDigits,
					GroupKind::AllDigits,
					500.into(),
				),
			);

			assert(
				&fixture.prize[1],
				&Record::new(
					Kind::Adjacent,
					NumberKind::AllDigits,
					GroupKind::AllDigits,
					5000.into(),
				),
			);
		}

		#[test]
		fn name() {
			let fixture = Spec::new("Name".into(), vec![]);
			assert_eq!(fixture.name(), "Name");
		}

		#[test]
		fn prize() {
			let fixture = Spec::new(
				"Name".into(),
				vec![Record::new(
					Kind::First,
					NumberKind::AllDigits,
					GroupKind::AllDigits,
					500.into(),
				)],
			);
			assert_eq!(fixture.prize().len(), 1);
			assert(&fixture.prize[0], &fixture.prize()[0]);
		}
	}
	mod record {
		use super::*;

		fn generate() -> Record {
			Record {
				kind: Kind::Fifth,
				number_kind: NumberKind::AllDigits,
				group_kind: GroupKind::None,
				prize: 200.into(),
			}
		}
		#[test]
		fn new() {
			let fixture = Record::new(
				Kind::Fifth,
				NumberKind::AllDigits,
				GroupKind::None,
				100.into(),
			);

			assert_eq!(fixture.kind, Kind::Fifth);
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
