use driver::formatter::ThousandsSeparator;
use implementation::lottery::Spec;
use implementation::prelude::drawing_machine::draw;
use implementation::prelude::*;
use rand::prelude::*;
use rust_decimal::prelude::*;

fn main() {
	let annual = annual();

	let mut rng = rand::rng();

	for spec in annual {
		proc(&spec, &mut rng);
		println!();
	}
}

fn proc(spec: &Spec, rng: &mut ThreadRng) {
	let mut shelf = if spec.group_size() == 100 {
		shelf::Shelf::build(10, 1..=100)
	} else if spec.group_size() == 200 {
		shelf::Shelf::build(10, 1..=200)
	} else {
		unreachable!()
	};

	let outcome = draw(spec, rng);

	let mut accum = rust_decimal::Decimal::new(0, 1);

	for t in shelf.iter_mut().flatten() {
		let result = outcome.verify(&t);
		accum += result.records().iter().map(|x| x.payout()).sum::<Decimal>();
	}

	println!("{}:￥{}", spec.name(), accum.separated());
}
