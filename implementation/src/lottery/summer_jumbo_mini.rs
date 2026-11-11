use super::spec::{Record, Spec};
use crate::PrizeCategory;
use crate::draw::{group::Kind as GroupKind, number::Kind as NumberKind};

pub fn summer_jumbo_mini() -> Spec {
	let v = vec![
		Record::new(
			PrizeCategory::First,
			NumberKind::AllDigits,
			GroupKind::LastOneDigit,
			30_000_000.into(),
		),
		Record::new(
			PrizeCategory::FirstAdjacent,
			NumberKind::AllDigits,
			GroupKind::None,
			10_000_000.into(),
		),
		Record::new(
			PrizeCategory::FirstAdjacent,
			NumberKind::AllDigits,
			GroupKind::None,
			10_000_000.into(),
		),
		Record::new(
			PrizeCategory::Second,
			NumberKind::LastThreeDigits,
			GroupKind::None,
			10_000.into(),
		),
		Record::new(
			PrizeCategory::Second,
			NumberKind::LastThreeDigits,
			GroupKind::None,
			10_000.into(),
		),
		Record::new(
			PrizeCategory::Third,
			NumberKind::LastTwoDigits,
			GroupKind::None,
			3_000.into(),
		),
		Record::new(
			PrizeCategory::Fourth,
			NumberKind::LastOneDigit,
			GroupKind::None,
			300.into(),
		),
	];
	Spec::new("サマージャンボミニ宝くじ".into(), 100, 300.into(), v)
}
