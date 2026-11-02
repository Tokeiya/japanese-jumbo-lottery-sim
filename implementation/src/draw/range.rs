#![expect(unused_variables, reason = "Under construction")]
#![expect(unfulfilled_lint_expectations, reason = "Under construction")]

use rand::Rng;
use rand::distr::uniform::{Error, SampleRange};

mod group {
	use super::*;
	pub struct AllRange100;

	impl SampleRange<u8> for AllRange100 {
		fn sample_single<R: Rng + ?Sized>(self, rng: &mut R) -> Result<u8, Error> {
			(1..=100).sample_single(rng)
		}

		fn is_empty(&self) -> bool {
			false
		}
	}

	pub struct AllRange200;

	impl SampleRange<u8> for AllRange200 {
		fn sample_single<R: Rng + ?Sized>(self, rng: &mut R) -> Result<u8, Error> {
			(1..=200).sample_single(rng)
		}

		fn is_empty(&self) -> bool {
			false
		}
	}

	pub struct LastOneDigit;

	impl SampleRange<u8> for LastOneDigit {
		fn sample_single<R: Rng + ?Sized>(self, rng: &mut R) -> Result<u8, Error> {
			todo!()
		}

		fn is_empty(&self) -> bool {
			todo!()
		}
	}
}

mod number {
	use super::*;

	pub struct AllRange;

	impl SampleRange<u32> for AllRange {
		fn sample_single<R: Rng + ?Sized>(self, rng: &mut R) -> Result<u32, Error> {
			(100_000..200_000).sample_single(rng)
		}
		fn is_empty(&self) -> bool {
			false
		}
	}

	pub struct LastFourDigits;

	impl SampleRange<u32> for LastFourDigits {
		fn sample_single<R: Rng + ?Sized>(self, rng: &mut R) -> Result<u32, Error> {
			todo!()
		}

		fn is_empty(&self) -> bool {
			todo!()
		}
	}

	pub struct LastThreeDigits;

	impl SampleRange<u32> for LastThreeDigits {
		fn sample_single<R: Rng + ?Sized>(self, rng: &mut R) -> Result<u32, Error> {
			todo!()
		}

		fn is_empty(&self) -> bool {
			todo!()
		}
	}

	pub struct LastTwoDigits;

	impl SampleRange<u32> for LastTwoDigits {
		fn sample_single<R: Rng + ?Sized>(self, rng: &mut R) -> Result<u32, Error> {
			todo!()
		}

		fn is_empty(&self) -> bool {
			todo!()
		}
	}

	pub struct LastOneDigit;

	impl SampleRange<u32> for LastOneDigit {
		fn sample_single<R: Rng + ?Sized>(self, rng: &mut R) -> Result<u32, Error> {
			todo!()
		}

		fn is_empty(&self) -> bool {
			todo!()
		}
	}
}
#[cfg(test)]
mod tests {
	use super::group::*;
	use super::number::*;
	use super::*;

	const ITERATION: usize = 1_000_000;

	#[test]
	fn test_group_range_100() {
		let mut rng = rand::rng();

		for _ in 0..ITERATION {
			let result = AllRange100.sample_single(&mut rng).unwrap();
			assert!((1..=100).contains(&result));
		}

		assert!(!AllRange100.is_empty())
	}

	#[test]
	fn test_group_range_200() {
		let mut rng = rand::rng();

		for _ in 0..ITERATION {
			let result = AllRange200.sample_single(&mut rng).unwrap();
			assert!((1..=200).contains(&result));
		}
		assert!(!AllRange200.is_empty());
	}

	#[test]
	fn test_number_range() {
		let mut rng = rand::rng();

		for _ in 0..ITERATION {
			let result = super::number::AllRange.sample_single(&mut rng).unwrap();
			assert!((100_000..200_000).contains(&result));
		}
		assert!(!AllRange.is_empty());
	}
}
