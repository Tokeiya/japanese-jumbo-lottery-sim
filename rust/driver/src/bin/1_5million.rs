use driver::player::{Player, Strategy};
use driver::random_store::{Store, take_sequence};
use dsv_writer::{Encoder, QuoteMode, ToDsv};
use implementation::lottery::Spec;
use implementation::prelude::*;
use rand::Rng;
use rand::prelude::IndexedRandom;
use rand::seq::IndexedMutRandom;
use rayon::prelude::*;
use std::fs::File;

fn main() {}

fn grp_100(year: u8, spec: &Spec, rng: &mut impl Rng, players: &[Player]) {
	let mut shelf = Shelf::build_with_rand(1, 0..=100, rng);
	let mut rand_store = Store::new(
		10_000_000,
		&mut Shelf::build_with_rand(0, 0..=100, rng),
		rng,
	);

	let mut p = players
		.iter()
		.map(|p| {
			(
				match p.strategy() {
					Strategy::Random10 => rand_store.take_10().unwrap(),
					Strategy::Sequence10 => take_sequence(&mut shelf, rng),
				},
				p,
			)
		})
		.collect::<Vec<_>>();

	let outcome = drawing_machine::draw(spec, rng);

	for (tickets, player) in p.iter() {
		for t in tickets.iter() {
			let payout = outcome.verify(t);
			while !player.try_accumlate(&payout, spec.unit_price()) {
				std::hint::spin_loop();
			}
		}
	}
}

fn grp_200(year: u8, spec: &Spec, rng: &mut impl Rng, players: &[Player]) {
	todo!()
}

fn proc(year: u8, spec: &Spec, rng: &mut impl Rng, players: &[Player]) {
	println!("{}年 {}", year, spec.name());

	let mut shelf = if spec.group_size() == 100 {
		shelf::Shelf::build(10, 1..=100)
	} else if spec.group_size() == 200 {
		shelf::Shelf::build(10, 1..=200)
	} else {
		unreachable!()
	};

	let mut rand_store = Store::new(5_000_000, &mut shelf, rng);

	let mut p = Vec::new();

	for player in players.iter() {
		let tickets = match player.strategy() {
			Strategy::Random10 => rand_store.take_10().unwrap(),
			Strategy::Sequence10 => take_sequence(&mut shelf, rng),
		};

		p.push((tickets, player))
	}

	let outcome = drawing_machine::draw(spec, rng);

	for (tickets, player) in p.iter() {
		for t in tickets.iter() {
			let payout = outcome.verify(t);

			while !player.try_accumlate(&payout, spec.unit_price()) {
				std::hint::spin_loop();
			}
		}
	}
}
