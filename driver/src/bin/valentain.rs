use implementation::{counter, draw, lottery,};

fn main() {
	let mut rnd = rand::rng();

	let mut shelf = counter::Shelf::build(10, 1..=100);

	let spec = lottery::valentine_mini();
	let outcome = draw::drawing_machine::draw(&spec, &mut rnd);

	let mut accum: rust_decimal::Decimal = 0.into();

	for ticket in shelf.iter_mut().flatten() {
		let result = outcome.verify(&ticket);
		for p in result.records().iter().map(|x| x.payout()) {
			accum += p;
		}
	}

	dbg!(accum);
}
