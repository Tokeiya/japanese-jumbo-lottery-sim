use super::draw::Report;
use rand::distr::uniform::SampleRange;
pub trait Lottery {
	fn group_range() -> impl SampleRange<u8>;
	fn number_range() -> impl SampleRange<u32>;
	fn name() -> &'static str;
	fn draw(&self, rng: &mut impl rand::Rng) -> Report;
}
