use super::spec::{Record, Spec};
use crate::PrizeCategory;
use crate::draw::{group::Kind as GroupKind, number::Kind as NumberKind};

pub fn halloween_jumbo() -> Spec {
	let v = vec![
		Record::new(
			PrizeCategory::First,
			NumberKind::AllDigits,
			GroupKind::AllDigits,
			200_000_000.into(),
		),
		Record::new(
			PrizeCategory::FirstAdjacent,
			NumberKind::AllDigits,
			GroupKind::None,
			100_000_000.into(),
		),
		Record::new(
			PrizeCategory::FirstAdjacent,
			NumberKind::AllDigits,
			GroupKind::None,
			100_000_000.into(),
		),
		Record::new(
			PrizeCategory::FirstDifferentGroup,
			NumberKind::AllDigits,
			GroupKind::None,
			100_000.into(),
		),
		Record::new(
			PrizeCategory::Second,
			NumberKind::AllDigits,
			GroupKind::AllDigits,
			100_000_000.into(),
		),
		Record::new(
			PrizeCategory::Second,
			NumberKind::AllDigits,
			GroupKind::AllDigits,
			100_000_000.into(),
		),
		Record::new(
			PrizeCategory::Third,
			NumberKind::AllDigits,
			GroupKind::LastOneDigit,
			1_000_000.into(),
		),
		Record::new(
			PrizeCategory::Fourth,
			NumberKind::LastThreeDigits,
			GroupKind::None,
			10_000.into(),
		),
		Record::new(
			PrizeCategory::Fourth,
			NumberKind::LastThreeDigits,
			GroupKind::None,
			10_000.into(),
		),
		Record::new(
			PrizeCategory::Fifth,
			NumberKind::LastTwoDigits,
			GroupKind::None,
			3_000.into(),
		),
		Record::new(
			PrizeCategory::Sixth,
			NumberKind::LastOneDigit,
			GroupKind::None,
			300.into(),
		),
	];
	Spec::new("ハロウィンジャンボ宝くじ".into(), 100, 200.into(), v)
}
