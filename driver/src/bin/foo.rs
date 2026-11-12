use driver::player::{Player, Strategy};
use driver::random_store::Store;
use implementation::lottery::Spec;
use implementation::prelude::*;
use rand::Rng;
use rand::seq::IndexedMutRandom;
use std::fs::File;

fn main() {
	let _annual = annual();
	let _rng = rand::rng();

	let mut sequence = Vec::new();
	let mut random = Vec::new();

	for i in 0..500_000 {
		sequence.push(Player::new(i, Strategy::Sequence10));
		random.push(Player::new(i, Strategy::Random10));
	}

	for year in 0..2 {
		println!("Year:{year}");

		for spec in annual() {
			proc(&spec, &mut rand::rng(), &mut sequence, &mut random);
		}
		println!();
	}

	let file = File::create("outpt.tsv").unwrap();
	let mut writer = dsv_writer::RawWriter::try_new(file, '\t', dsv_writer::NewLine::CrLf);
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

fn proc(spec: &Spec, rng: &mut impl Rng, sequence: &mut [Player], random: &mut [Player]) {
	println!("{}", spec.name());

	let mut shelf = if spec.group_size() == 100 {
		shelf::Shelf::build(10, 1..=100)
	} else if spec.group_size() == 200 {
		shelf::Shelf::build(10, 1..=200)
	} else {
		unreachable!()
	};

	let mut rand_store = Store::new(5_000_000, &mut shelf, rng);

	let mut s = Vec::with_capacity(sequence.len());

	for person in sequence.iter_mut() {
		s.push((take_sequence(&mut shelf, rng), person));
	}

	let mut r = Vec::with_capacity(random.len());

	for person in random.iter_mut() {
		r.push((rand_store.take_10().unwrap(), person));
	}

	let outcome = drawing_machine::draw(spec, rng);

	for (tickets, player) in s.iter() {
		for t in tickets.iter() {
			let payout = outcome.verify(t);

			while !player.try_accumlate(&payout, spec.unit_price()) {
				std::hint::spin_loop();
			}
		}
	}

	for (tickets, player) in r.iter() {
		for t in tickets.iter() {
			let payout = outcome.verify(t);

			while !player.try_accumlate(&payout, spec.unit_price()) {
				std::hint::spin_loop();
			}
		}
	}
}
