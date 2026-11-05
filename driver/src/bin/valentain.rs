use implementation::ticket::Ticket;
use implementation::{counter, draw, lottery, ticket};

fn main() {
	let mut rnd = rand::rng();

	let mut shelf = counter::Shelf::build(10, 1..=100);
	let mut cnt = 0usize;

	let spec = lottery::valentine();
	let outcome = draw::drawing_machine::draw(&spec, &mut rnd);

	let mut accum: rust_decimal::Decimal = 0.into();

	for ticket in shelf.iter_mut().flatten() {
		let result = outcome.verify(&ticket);
		for p in result.records().iter().map(|x| x.payout()) {
			accum += p;
		}
	}

	dbg!(cnt);
	dbg!(accum);
}
