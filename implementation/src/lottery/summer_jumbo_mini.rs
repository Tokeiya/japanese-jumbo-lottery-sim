use super::spec::{Record, Spec};
use crate::PrizeCategory;
use crate::draw::{group::Kind as GroupKind, number::Kind as NumberKind};
use std::sync::LazyLock;

pub static SUMMER_JUMBO_MINI: LazyLock<Spec> = LazyLock::new(|| {
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
	Spec::new("サマージャンボミニ宝くじ".into(), v)
});
