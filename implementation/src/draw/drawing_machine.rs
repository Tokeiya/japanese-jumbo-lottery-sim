use super::group::Group;
use super::number::Number;
use rand::{Rng, RngExt};

pub struct DrawingMachine<R: Rng>(R);

impl<R: Rng> From<R> for DrawingMachine<R> {
	fn from(rng: R) -> Self {
		Self(rng)
	}
}

impl<R: Rng> DrawingMachine<R> {
	pub fn draw_group(&mut self, group: &mut Group) {
		todo!()
	}

	pub fn draw_number(&mut self, number: &mut Number) {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use mockall::mock;
	use rand::{Rng, TryRng};
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

	#[test]
	fn from() {
		let mut mock = MockRng::new();
		let mut seq = mockall::Sequence::new();

		let tmp = mock.expect_try_next_u32();
		tmp.in_sequence(&mut seq).return_const(Ok(42)).times(1);
		tmp.in_sequence(&mut seq).return_const(Ok(114514)).times(1);
		tmp.in_sequence(&mut seq).return_const(Ok(810)).times(1);

		let mut fixture = DrawingMachine::from(mock);

		assert_eq!(fixture.0.next_u32(), 42);
		assert_eq!(fixture.0.next_u32(), 114514);
		assert_eq!(fixture.0.next_u32(), 810);
	}

	#[test]
	fn draw_number() {
		todo!();
	}

	#[test]
	fn draw_group() {
		let mut mock = MockRng::new();

		mock.expect_try_next_u32()
			.return_const(Ok(rand::rng().next_u32()));

		let expected1 = mock.random_range(0..10u8);
		let expected2 = mock.random_range(1..=100u8);

		let mut fixture = DrawingMachine::from(mock);

		let mut act = Group::last_one_digits(0);

		fixture.draw_group(&mut act);

		if let Group::Last1Digits(x) = act {
			assert_eq!(x, expected1);
		} else {
			unreachable!()
		}

		let mut act = Group::all_digits(0);

		fixture.draw_group(&mut act);

		if let Group::AllDigits(x) = act {
			assert_eq!(x, expected2);
		} else {
			unreachable!()
		}

		let mut act = Group::None;

		fixture.draw_group(&mut act);
	}
}
