use super::group::{self, Group};
use super::number::{self, Number};
use rand::{Rng, RngExt};

pub fn draw_group_100(rng: &mut impl Rng, group: group::Kind) -> Group {
	todo!()
}

pub fn draw_group_200(rng: &mut impl Rng, group: group::Kind) -> Group {
	todo!()
}

pub fn draw_number(rng: &mut impl Rng, number: number::Kind) -> Number {
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
			let expected = rng.random_range(1..100u8);

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
			let expected = rng.random_range(1..100u8);

			assert_matches!(actual.kind(), group::Kind::AllDigits);
			assert_matches!(actual.value(),Some(x) if x==expected);

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
