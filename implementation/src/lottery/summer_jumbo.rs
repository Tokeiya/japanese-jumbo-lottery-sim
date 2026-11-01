use super::spec::{Record, Spec};
use crate::PrizeCategory;
use crate::draw::{group::Kind as GroupKind, number::Kind as NumberKind};
use std::sync::LazyLock;

pub static SUMMER_JUMBO: LazyLock<Spec> = LazyLock::new(|| {
	let v = vec![
		Record::new(
			PrizeCategory::First,
			NumberKind::AllDigits,
			GroupKind::AllDigits,
			500_000_000.into(),
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
	Spec::new("サマージャンボ宝くじ".into(), v)
});
