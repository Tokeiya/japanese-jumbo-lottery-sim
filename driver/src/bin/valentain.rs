use driver::formatter::*;
use implementation::{PrizeCategory, counter, draw, lottery};
use rust_decimal::Decimal;
use std::collections::HashMap;
fn main() {
	let mut rnd = rand::rng();
	let mut shelf = counter::Shelf::build(10, 1..=200);

	let mut accum: HashMap<PrizeCategory, Decimal> = std::collections::HashMap::new();

	let arr = [
		PrizeCategory::First,
		PrizeCategory::FirstAdjacent,
		PrizeCategory::FirstDifferentGroup,
		PrizeCategory::Second,
		PrizeCategory::Third,
		PrizeCategory::Fourth,
		PrizeCategory::Fifth,
		PrizeCategory::Sixth,
		PrizeCategory::Seventh,
	];

	for p in arr.iter() {
		accum.insert(*p, 0.into());
	}

	let spec = lottery::year_end_jumbo();
	let outcome = draw::drawing_machine::draw(&spec, &mut rnd);

	for ticket in shelf.iter_mut().flatten() {
		let result = outcome.verify(&ticket);

		for o in result.records() {
			*accum.get_mut(&o.rank()).unwrap() += o.payout();
		}
	}

	for p in arr.iter() {
		println!("{:?}:￥{}", p, accum.get(p).unwrap().separated());
	}

	println!("TTL:{}", accum.values().sum::<Decimal>().separated());
}
