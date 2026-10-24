// use super::group::GroupKind;
// use super::number::Number;
// use rand::{Rng, RngExt};
//
// pub fn draw_group(rng: &mut impl Rng, group: GroupKind)->GroupKind {
// 	todo!()
// }
//
// pub fn draw_number(rng: &mut impl Rng, number: Number) -> Number {
// 	todo!()
// }
//
// #[cfg(test)]
// mod tests {
// 	use super::*;
// 	use mockall::mock;
// 	use rand::{Rng, TryRng};
// 	use std::convert::Infallible;
//
// 	mock! {
// 		pub Rng {}
//
// 		impl TryRng for Rng {
// 			type Error = Infallible;
//
// 			fn try_next_u32(&mut self) -> Result<u32, Infallible>;
// 			fn try_next_u64(&mut self) -> Result<u64, Infallible>;
// 			fn try_fill_bytes(
// 				&mut self,
// 				dst: &mut [u8],
// 			) -> Result<(), Infallible>;
// 		}
// 	}
//
// 	fn generate(repeat: usize) -> MockRng {
// 		let mut mock = MockRng::new();
// 		let mut cnt = repeat;
// 		let mut rng = rand::rng();
// 		let mut register = 0;
//
// 		mock.expect_try_next_u32().returning_st(move || {
// 			cnt += 1;
//
// 			if cnt < repeat {
// 				Ok(register)
// 			} else {
// 				cnt = 0;
// 				register = rng.next_u32();
// 				Ok(register)
// 			}
// 		});
//
// 		mock
// 	}
//
// 	#[test]
// 	fn mock() {
// 		let mut fixture = generate(2);
//
// 		let mut recent = 0;
//
// 		for i in 0..10 {
// 			if i % 2 == 0 {
// 				recent = fixture.next_u32();
// 			} else {
// 				assert_eq!(fixture.next_u32(), recent)
// 			}
// 		}
// 	}
// 	#[test]
// 	fn draw_number() {
// 		let mut rng = generate(2);
// 	}
// 	#[test]
// 	fn draw_group() {
// 		let mut g = GroupKind::last_one_digits(10);
// 		g = GroupKind::all_digits(20);
//
// 		g = GroupKind::None;
//
// 		todo!()
// 	}
// }
