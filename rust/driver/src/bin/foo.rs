use driver::player::{Player, Strategy};
use driver::random_store::Store;
use dsv_writer::{Encoder, QuoteMode, ToDsv};
use implementation::lottery::Spec;
use implementation::prelude::*;
use rand::Rng;
use rand::seq::IndexedMutRandom;
use rayon::prelude::*;
use std::fs::File;

fn main() {
	let _annual = annual();
	let _rng = rand::rng();

	let mut players = Vec::new();

	for i in 0..500_000 {
		players.push(Player::new(i, Strategy::Sequence10));
	}

	for i in 500_000..1_000_000 {
		players.push(Player::new(i, Strategy::Random10));
	}

	let a = annual();

	(0..60).into_par_iter().for_each(|year| {
		for spec in annual() {
			proc(year, &spec, &mut rand::rng(), &players)
		}
	});

	let file = File::create("outpt.tsv").unwrap();
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

fn take_sequence(shelf: &mut Shelf, rng: &mut impl Rng) -> [Ticket; 10] {
	let mut cnt = 0usize;
	let grp = loop {
		shelf.trim();
		let tmp = shelf.racks_mut().choose_mut(rng).unwrap();
		if tmp.len() < 10 {
			cnt += 1;
			assert!(cnt > 100, "Reach Threshold");
			continue;
		} else {
			break tmp;
		}
	};

	std::array::from_fn(|_| grp.next().unwrap())
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
