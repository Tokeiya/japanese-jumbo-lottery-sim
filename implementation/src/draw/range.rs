use rand::Rng;
use rand::distr::uniform::{Error, SampleRange};
pub struct GroupRange100;

impl SampleRange<u8> for GroupRange100 {
	fn sample_single<R: Rng + ?Sized>(self, rng: &mut R) -> Result<u8, Error> {
		(1..=100).sample_single(rng)
	}

	fn is_empty(&self) -> bool {
		false
	}
}

pub struct GroupRange200;

impl SampleRange<u8> for GroupRange200 {
	fn sample_single<R: Rng + ?Sized>(self, rng: &mut R) -> Result<u8, Error> {
		(1..=200).sample_single(rng)
	}

	fn is_empty(&self) -> bool {
		false
	}
}

pub struct NumberRange;

impl SampleRange<u32> for NumberRange {
	fn sample_single<R: Rng + ?Sized>(self, rng: &mut R) -> Result<u32, Error> {
		(100_000..200_000).sample_single(rng)
	}
	fn is_empty(&self) -> bool {
		false
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const ITERATION: usize = 1_000_000;

	#[test]
	fn test_group_range_100() {
		let mut rng = rand::rng();

		for _ in 0..ITERATION {
			let result = GroupRange100.sample_single(&mut rng).unwrap();
			assert!((1..=100).contains(&result));
		}

		assert!(!GroupRange100.is_empty())
	}

	#[test]
	fn test_group_range_200() {
		let mut rng = rand::rng();

		for _ in 0..ITERATION {
			let result = GroupRange200.sample_single(&mut rng).unwrap();
			assert!((1..=200).contains(&result));
		}
		assert!(!GroupRange200.is_empty());
	}

	#[test]
	fn test_number_range() {
		let mut rng = rand::rng();

		for _ in 0..ITERATION {
			let result = NumberRange.sample_single(&mut rng).unwrap();
			assert!((100_000..200_000).contains(&result));
		}
		assert!(!NumberRange.is_empty());
	}
}
