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
	todo!()
}

fn previous_number(number: &Number) -> Number {
	todo!()
}

fn find_first_adjacent(scr: &[Record]) -> &Record {
	todo!()
}

fn find_different_group(scr: &[Record]) -> &Record {
	todo!()
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
		num,
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

	for r in spec.prize().iter() {
		match r.kind() {
			PrizeCategory::First => {}
			PrizeCategory::FirstAdjacent => {}
			PrizeCategory::FirstDifferentGroup => {}
			PrizeCategory::Second => {}
			PrizeCategory::Third => {}
			PrizeCategory::Fourth => {}
			PrizeCategory::Fifth => {}
			PrizeCategory::Sixth => {}
			PrizeCategory::Seventh => {}
		}
	}

	todo!()
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
}
