use super::spec::{Record, Spec};
use crate::PrizeCategory;
use crate::draw::{group::Kind as GroupKind, number::Kind as NumberKind};
use std::sync::LazyLock;

pub static YEAR_END_JUMBO_MINI: LazyLock<Spec> = LazyLock::new(|| {
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
			NumberKind::AllDigits,
			GroupKind::LastOneDigit,
			1_000_000.into(),
		),
		Record::new(
			PrizeCategory::Third,
			NumberKind::LastFourDigits,
			GroupKind::None,
			30_000.into(),
		),
		Record::new(
			PrizeCategory::Third,
			NumberKind::LastFourDigits,
			GroupKind::None,
			30_000.into(),
		),
		Record::new(
			PrizeCategory::Third,
			NumberKind::LastFourDigits,
			GroupKind::None,
			30_000.into(),
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
	Spec::new("年末ジャンボミニ宝くじ".into(), 100, 200.into(), v)
});
