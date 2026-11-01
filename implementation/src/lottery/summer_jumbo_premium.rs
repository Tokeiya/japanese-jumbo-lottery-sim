use super::spec::{Record, Spec};
use crate::PrizeCategory;
use crate::draw::{group::Kind as GroupKind, number::Kind as NumberKind};
use std::sync::LazyLock;

pub static SUMMER_JUMBO_PREMIUM: LazyLock<Spec> = LazyLock::new(|| {
	let v = vec![
		Record::new(
			PrizeCategory::First,
			NumberKind::AllDigits,
			GroupKind::AllDigits,
			800_000_000.into(),
		),
		Record::new(
			PrizeCategory::FirstAdjacent,
			NumberKind::AllDigits,
			GroupKind::None,
			200_000_000.into(),
		),
		Record::new(
			PrizeCategory::FirstAdjacent,
			NumberKind::AllDigits,
			GroupKind::None,
			200_000_000.into(),
		),
		Record::new(
			PrizeCategory::FirstDifferentGroup,
			NumberKind::AllDigits,
			GroupKind::None,
			500_000.into(),
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
			NumberKind::LastTwoDigits,
			GroupKind::None,
			5_000.into(),
		),
		Record::new(
			PrizeCategory::Fourth,
			NumberKind::LastOneDigit,
			GroupKind::None,
			500.into(),
		),
	];
	Spec::new("サマージャンボプレミアム".into(), v)
});
