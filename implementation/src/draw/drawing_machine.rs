use super::group::{self, Group};
use super::number::{self, Number};
use super::outcome::{Outcome, Record as OutcomeRecord};
use crate::PrizeCategory;
use crate::lottery::{Record, Spec};
use rand::{Rng, RngExt};

pub fn draw_group_100(rng: &mut impl Rng, group: group::Kind) -> Group {
	match group {
		group::Kind::AllDigits => Group::all_digits(rng.random_range(1..=100u8)),
		group::Kind::LastOneDigit => Group::last_one_digit(rng.random_range(0..10u8)),
		group::Kind::None => Group::none(),
	}
}

pub fn draw_group_200(rng: &mut impl Rng, group: group::Kind) -> Group {
	match group {
		group::Kind::AllDigits => Group::all_digits(rng.random_range(1..=200u8)),
		group::Kind::LastOneDigit => Group::last_one_digit(rng.random_range(0..10u8)),
		group::Kind::None => Group::none(),
	}
}

pub fn draw_number(rng: &mut impl Rng, number: number::Kind) -> Number {
	match number {
		number::Kind::AllDigits => Number::all_digits(rng.random_range(100_000u32..200_000)),
		number::Kind::LastFourDigits => Number::last_four_digits(rng.random_range(0u32..10_000)),
		number::Kind::LastThreeDigits => Number::last_three_digits(rng.random_range(0..1_000u32)),
		number::Kind::LastTwoDigits => Number::last_two_digits(rng.random_range(0..100u32)),
		number::Kind::LastOneDigit => Number::last_one_digit(rng.random_range(0..10u32)),
	}
}

fn next_number(number: &Number) -> Number {
	if number.kind() != number::Kind::AllDigits {
		panic!("Only all digits numbers can be incremented")
	}

	if number.value() == 199_999 {
		Number::all_digits(100_000)
	} else {
		Number::all_digits(number.value() + 1)
	}
}

fn previous_number(number: &Number) -> Number {
	if number.kind() != number::Kind::AllDigits {
		panic!("Only all digits numbers can be decremented")
	}

	if number.value() == 100_000 {
		Number::all_digits(199_999)
	} else {
		Number::all_digits(number.value() - 1)
	}
}

fn find_first_adjacent(scr: &[Record]) -> &Record {
	let vec: Vec<_> = scr
		.iter()
		.filter(|&x| x.kind() == PrizeCategory::FirstAdjacent)
		.collect();
	if vec.len() != 2 {
		panic!("First adjacent prize must be unique")
	}

	if vec[0].prize() != vec[1].prize() {
		panic!("First adjacent prize must be unique")
	}

	vec[0]
}

fn find_different_group(scr: &[Record]) -> Option<&Record> {
	let vec: Vec<_> = scr
		.iter()
		.filter(|&x| x.kind() == PrizeCategory::FirstDifferentGroup)
		.collect();

	if vec.len() == 0 {
		None
	} else if vec.len() != 1 {
		panic!("First different group prize must be unique")
	} else {
		vec[0].into()
	}
}

fn draw_impl(datum: &Record, group_size: u8, rng: &mut impl Rng) -> OutcomeRecord {
	let grp = if group_size == 100 {
		draw_group_100(rng, datum.group_kind())
	} else if group_size == 200 {
		draw_group_200(rng, datum.group_kind())
	} else {
		panic!("Invalid group size")
	};

	let num = draw_number(rng, datum.number_kind());

	OutcomeRecord::new(datum.kind(), datum.prize(), grp, num)
}

pub fn draw(spec: &Spec, rng: &mut impl Rng) -> Outcome {
	let mut vec: Vec<OutcomeRecord> = Vec::new();

	let first: Vec<&Record> = spec
		.prize()
		.iter()
		.filter(|&x| x.kind() == PrizeCategory::First)
		.collect();

	if first.len() != 1 {
		panic!("First prize must be unique")
	}

	let first = first[0];

	let grp = if spec.group_size() == 100 {
		draw_group_100(rng, first.group_kind())
	} else if spec.group_size() == 200 {
		draw_group_200(rng, first.group_kind())
	} else {
		panic!("Invalid group size")
	};

	let num = draw_number(rng, first.number_kind());

	let next_num = next_number(&num);
	let previous_num = previous_number(&num);

	let adjacent_prize = find_first_adjacent(spec.prize()).prize();

	vec.push(OutcomeRecord::new(
		PrizeCategory::First,
		first.prize(),
		grp.clone(),
		num.clone(),
	));

	vec.push(OutcomeRecord::new(
		PrizeCategory::FirstAdjacent,
		adjacent_prize,
		grp.clone(),
		next_num,
	));

	vec.push(OutcomeRecord::new(
		PrizeCategory::FirstAdjacent,
		adjacent_prize,
		grp.clone(),
		previous_num,
	));

	if let Some(rec) = find_different_group(spec.prize()) {
		vec.push(OutcomeRecord::new(
			PrizeCategory::FirstDifferentGroup,
			rec.prize(),
			Group::none(),
			num,
		))
	}

	for r in spec.prize().iter() {
		match r.kind() {
			PrizeCategory::First => continue,
			PrizeCategory::FirstAdjacent => continue,
			PrizeCategory::FirstDifferentGroup => continue,
			_ => vec.push(draw_impl(r, spec.group_size(), rng)),
		}
	}

	Outcome::new(vec)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::draw::{group, number};
	use mockall::mock;
	use rand::{Rng, TryRng};
	use std::assert_matches;
	use std::convert::Infallible;

	mock! {
		pub Rng {}

		impl TryRng for Rng {
			type Error = Infallible;

			fn try_next_u32(&mut self) -> Result<u32, Infallible>;
			fn try_next_u64(&mut self) -> Result<u64, Infallible>;
			fn try_fill_bytes(
				&mut self,
				dst: &mut [u8],
			) -> Result<(), Infallible>;
		}
	}

	fn generate(repeat: usize) -> MockRng {
		let mut mock = MockRng::new();
		let mut cnt = repeat;
		let mut rng = rand::rng();
		let mut register = 0;

		mock.expect_try_next_u32().returning_st(move || {
			cnt += 1;

			if cnt < repeat {
				Ok(register)
			} else {
				cnt = 0;
				register = rng.next_u32();
				Ok(register)
			}
		});

		mock
	}

	#[test]
	fn mock() {
		let mut fixture = generate(2);

		let mut recent = 0;

		for i in 0..10 {
			if i % 2 == 0 {
				recent = fixture.next_u32();
			} else {
				assert_eq!(fixture.next_u32(), recent)
			}
		}
	}

	const ITERATION: usize = 10;
	#[test]
	fn draw_number() {
		let mut rng = generate(2);

		for _ in 0..ITERATION {
			let actual = super::draw_number(&mut rng, number::Kind::AllDigits);
			let expected = rng.random_range(100_000u32..200_000);

			assert_matches!(actual.kind(), number::Kind::AllDigits);
			assert_eq!(actual.value(), expected);
		}

		for _ in 0..ITERATION {
			let actual = super::draw_number(&mut rng, number::Kind::LastFourDigits);
			let expected = rng.random_range(0u32..10_000);

			assert_matches!(actual.kind(), number::Kind::LastFourDigits);
			assert_eq!(actual.value(), expected);
		}

		for _ in 0..ITERATION {
			let actual = super::draw_number(&mut rng, number::Kind::LastThreeDigits);
			let expected = rng.random_range(0..1_000u32);
			assert_matches!(actual.kind(), number::Kind::LastThreeDigits);
			assert_eq!(actual.value(), expected);
		}

		for _ in 0..ITERATION {
			let actual = super::draw_number(&mut rng, number::Kind::LastTwoDigits);
			let expected = rng.random_range(0..100u32);
			assert_matches!(actual.kind(), number::Kind::LastTwoDigits);
			assert_eq!(actual.value(), expected);
		}
	}
	#[test]
	fn draw_group_100() {
		let mut rng = generate(2);

		for _ in 0..ITERATION {
			let actual = super::draw_group_100(&mut rng, group::Kind::AllDigits);
			let expected = rng.random_range(1..=100u8);

			assert_matches!(actual.kind(), group::Kind::AllDigits);
			assert_matches!(actual.value(),Some(x) if x==expected);

			let actual = super::draw_group_100(&mut rng, group::Kind::LastOneDigit);
			let expected = rng.random_range(0..10u8);
			assert_matches!(actual.kind(), group::Kind::LastOneDigit);
			assert_matches!(actual.value(), Some(x) if x==expected);

			let actual = super::draw_group_100(&mut rng, group::Kind::None);
			assert_matches!(actual.kind(), group::Kind::None);
			assert_matches!(actual.value(), None);
		}
	}

	#[test]
	fn draw_group_200() {
		let mut rng = generate(2);

		for _ in 0..ITERATION {
			let actual = super::draw_group_200(&mut rng, group::Kind::AllDigits);
			let expected = rng.random_range(1..=200u8);

			assert_matches!(actual.kind(), group::Kind::AllDigits);
			assert_matches!(actual.value(),Some(x) if x==expected,"act:{:?} exp:{expected}",actual.value());

			let actual = super::draw_group_200(&mut rng, group::Kind::LastOneDigit);
			let expected = rng.random_range(0..10u8);
			assert_matches!(actual.kind(), group::Kind::LastOneDigit);
			assert_matches!(actual.value(), Some(x) if x==expected);

			let actual = super::draw_group_200(&mut rng, group::Kind::None);
			assert_matches!(actual.kind(), group::Kind::None);
			assert_matches!(actual.value(), None);
		}
	}

	#[test]
	fn next_number_test() {
		let piv = Number::all_digits(100_000);
		let fixture = next_number(&piv);

		assert_eq!(fixture.kind(), number::Kind::AllDigits);
		assert_eq!(fixture.value(), 100_001);

		let piv = Number::all_digits(199_999);
		let fixture = next_number(&piv);

		assert_eq!(fixture.kind(), number::Kind::AllDigits);
		assert_eq!(fixture.value(), 100_000);
	}

	#[test]
	fn invalid_next_number() {
		use std::panic::catch_unwind;

		let err = catch_unwind(|| _ = next_number(&Number::last_four_digits(4321)));

		err.unwrap_err();
	}

	#[test]
	fn previous_number_test() {
		use super::previous_number as target;
		use number::Kind;

		let piv = Number::all_digits(100_000);
		let fixture = target(&piv);

		assert_eq!(fixture.kind(), Kind::AllDigits);
		assert_eq!(fixture.value(), 199_999);

		let piv = Number::all_digits(199_999);
		let fixture = target(&piv);

		assert_eq!(fixture.kind(), Kind::AllDigits);
		assert_eq!(fixture.value(), 199_998);
	}

	#[test]
	fn invalid_previous_number() {
		use std::panic::catch_unwind;

		catch_unwind(|| {
			_ = previous_number(&Number::last_four_digits(4321));
		})
		.unwrap_err();
	}

	#[test]
	fn find_different_group_test() {
		let vec: Vec<Record> = vec![
			Record::new(
				PrizeCategory::FirstDifferentGroup,
				number::Kind::AllDigits,
				group::Kind::None,
				100.into(),
			),
			Record::new(
				PrizeCategory::First,
				number::Kind::AllDigits,
				group::Kind::AllDigits,
				200.into(),
			),
		];
		let fixture = find_different_group(&vec);

		assert!(std::ptr::eq(&vec[0], fixture.unwrap()));
	}

	#[test]
	#[should_panic]
	fn invalid_find_different_group() {
		let vec: Vec<Record> = vec![
			Record::new(
				PrizeCategory::FirstDifferentGroup,
				number::Kind::AllDigits,
				group::Kind::None,
				100.into(),
			),
			Record::new(
				PrizeCategory::FirstDifferentGroup,
				number::Kind::AllDigits,
				group::Kind::AllDigits,
				200.into(),
			),
		];

		_ = find_different_group(&vec);
	}

	#[test]
	fn find_first_adjacent_test() {
		let vec: Vec<Record> = vec![
			Record::new(
				PrizeCategory::FirstAdjacent,
				number::Kind::AllDigits,
				group::Kind::None,
				100.into(),
			),
			Record::new(
				PrizeCategory::FirstAdjacent,
				number::Kind::AllDigits,
				group::Kind::AllDigits,
				100.into(),
			),
			Record::new(
				PrizeCategory::First,
				number::Kind::AllDigits,
				group::Kind::AllDigits,
				1000.into(),
			),
		];

		let fixture = find_first_adjacent(&vec);

		assert_eq!(fixture.kind(), PrizeCategory::FirstAdjacent);
		assert_eq!(fixture.prize(), 100.into());
	}

	#[test]
	fn draw_test() {
		fn take_one<T>(mut iter: impl Iterator<Item = T>) -> T {
			let ret = iter.next().unwrap();
			assert!(iter.next().is_none());
			ret
		}

		let vec = vec![
			Record::new(
				PrizeCategory::FirstAdjacent,
				number::Kind::AllDigits,
				group::Kind::None,
				100.into(),
			),
			Record::new(
				PrizeCategory::FirstAdjacent,
				number::Kind::AllDigits,
				group::Kind::AllDigits,
				100.into(),
			),
			Record::new(
				PrizeCategory::First,
				number::Kind::AllDigits,
				group::Kind::AllDigits,
				1000.into(),
			),
			Record::new(
				PrizeCategory::FirstDifferentGroup,
				number::Kind::AllDigits,
				group::Kind::None,
				200.into(),
			),
			Record::new(
				PrizeCategory::Second,
				number::Kind::AllDigits,
				group::Kind::AllDigits,
				500.into(),
			),
		];

		let spec = Spec::new("Sample".to_string(), 100, 200.into(), vec);

		let mut rng = rand::rng();

		let fixture = draw(&spec, &mut rng);

		assert_eq!(fixture.records().len(), 5);

		let num = take_one(
			fixture
				.records()
				.iter()
				.filter(|x| x.rank() == PrizeCategory::First),
		)
		.number()
		.clone();

		let adjacent: Vec<_> = fixture
			.records()
			.iter()
			.filter(|x| x.rank() == PrizeCategory::FirstAdjacent)
			.collect();

		assert_eq!(adjacent.len(), 2);

		let expected = if num.value() == 199_999 {
			100_000
		} else {
			num.value() + 1
		};

		assert_eq!(
			adjacent
				.iter()
				.filter(|&x| x.number().value() == expected)
				.count(),
			1
		);

		let expected = if num.value() == 100_000 {
			199_999
		} else {
			num.value() - 1
		};

		assert_eq!(
			adjacent
				.iter()
				.filter(|x| x.number().value() == expected)
				.count(),
			1
		);
	}
}
