use super::spec::{Record, Spec};
use crate::PrizeCategory;
use crate::draw::{group::Kind as GroupKind, number::Kind as NumberKind};

pub fn halloween_jumbo_mini() -> Spec {
	let v = vec![
		Record::new(
			PrizeCategory::First,
			NumberKind::AllDigits,
			GroupKind::LastOneDigit,
			20_000_000.into(),
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
			NumberKind::AllDigits,
			GroupKind::LastOneDigit,
			1_000_000.into(),
		),
		Record::new(
			PrizeCategory::Second,
			NumberKind::AllDigits,
			GroupKind::LastOneDigit,
			1_000_000.into(),
		),
		Record::new(
			PrizeCategory::Third,
			NumberKind::LastThreeDigits,
			GroupKind::None,
			10_000.into(),
		),
		Record::new(
			PrizeCategory::Third,
			NumberKind::LastThreeDigits,
			GroupKind::None,
			10_000.into(),
		),
		Record::new(
			PrizeCategory::Third,
			NumberKind::LastThreeDigits,
			GroupKind::None,
			10_000.into(),
		),
		Record::new(
			PrizeCategory::Third,
			NumberKind::LastThreeDigits,
			GroupKind::None,
			10_000.into(),
		),
		Record::new(
			PrizeCategory::Fourth,
			NumberKind::LastTwoDigits,
			GroupKind::None,
			3_000.into(),
		),
		Record::new(
			PrizeCategory::Fifth,
			NumberKind::LastOneDigit,
			GroupKind::None,
			300.into(),
		),
	];
	Spec::new("ハロウィンジャンボミニ宝くじ".into(), 100, 300.into(), v)
}
