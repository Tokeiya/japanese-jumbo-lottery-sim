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

fn main() {
	let _annual = annual();
	let _rng = rand::rng();

	let mut players = Vec::new();

	for i in 0..750_000 {
		players.push(Player::new(i, Strategy::Sequence10));
	}

	for i in 750_000..1_500_000 {
		players.push(Player::new(i, Strategy::Random10));
	}

	(0..60).into_par_iter().for_each(|year| {
		for spec in annual() {
			proc(year, &spec, &mut rand::rng(), &players)
		}
	});

	let file = File::create("../15outpt.tsv").unwrap();
	let mut writer = dsv_writer::RawWriter::try_new(file, '\t', dsv_writer::NewLine::Lf).unwrap();

	writer.write_str_field("id", QuoteMode::AutoDetect).unwrap();
	writer
		.write_str_field("strategy", QuoteMode::AutoDetect)
		.unwrap();

	writer
		.write_str_field("total_payed", QuoteMode::AutoDetect)
		.unwrap();

	writer
		.write_str_field("first_cnt", QuoteMode::AutoDetect)
		.unwrap();
	writer
		.write_str_field("first_prize", QuoteMode::AutoDetect)
		.unwrap();

	writer
		.write_str_field("first_adjacent_cnt", QuoteMode::AutoDetect)
		.unwrap();
	writer
		.write_str_field("first_adjacent_prize", QuoteMode::AutoDetect)
		.unwrap();

	writer
		.write_str_field("first_different_group_cnt", QuoteMode::AutoDetect)
		.unwrap();
	writer
		.write_str_field("first_different_group_prize", QuoteMode::AutoDetect)
		.unwrap();

	writer
		.write_str_field("second_cnt", QuoteMode::AutoDetect)
		.unwrap();
	writer
		.write_str_field("second_prize", QuoteMode::AutoDetect)
		.unwrap();

	writer
		.write_str_field("third_cnt", QuoteMode::AutoDetect)
		.unwrap();
	writer
		.write_str_field("third_prize", QuoteMode::AutoDetect)
		.unwrap();

	writer
		.write_str_field("fourth_cnt", QuoteMode::AutoDetect)
		.unwrap();
	writer
		.write_str_field("fourth_prize", QuoteMode::AutoDetect)
		.unwrap();

	writer
		.write_str_field("fifth_cnt", QuoteMode::AutoDetect)
		.unwrap();
	writer
		.write_str_field("fifth_prize", QuoteMode::AutoDetect)
		.unwrap();

	writer
		.write_str_field("sixth_cnt", QuoteMode::AutoDetect)
		.unwrap();
	writer
		.write_str_field("sixth_prize", QuoteMode::AutoDetect)
		.unwrap();

	writer
		.write_str_field("seventh_cnt", QuoteMode::AutoDetect)
		.unwrap();
	writer
		.write_str_field("seventh_prize", QuoteMode::AutoDetect)
		.unwrap();

	writer.end_of_record(false).unwrap();

	for p in players.iter() {
		p.to_dsv(&mut writer).unwrap();
	}
}

fn grp_100(spec: &Spec, rng: &mut impl Rng, players: &[Player]) {
	let mut shelf = Shelf::build_with_rand(1, 0..=100, rng);
	let rand_store = Store::new(
		10_000_000,
		&mut Shelf::build_with_rand(0, 0..=100, rng),
		rng,
	);

	lottery_proc(spec, rng, players, &mut shelf, rand_store);
}

fn grp_200(spec: &Spec, rng: &mut impl Rng, players: &[Player]) {
	let mut shelf = Shelf::build_with_rand(0, 0..=200, rng);
	let rand_store = Store::new(10_000_000, &mut shelf, rng);

	lottery_proc(spec, rng, players, &mut shelf, rand_store);
}

fn lottery_proc(
	spec: &Spec,
	rng: &mut impl Rng,
	players: &[Player],
	mut shelf: &mut Shelf,
	mut rand_store: Store,
) {
	let p = players
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

fn proc(year: u8, spec: &Spec, rng: &mut impl Rng, players: &[Player]) {
	println!("{}年 {}", year, spec.name());

	if spec.group_size() == 100 {
		grp_100(spec, rng, players);
	} else if spec.group_size() == 200 {
		grp_200(spec, rng, players);
	} else {
		unreachable!()
	};
}
