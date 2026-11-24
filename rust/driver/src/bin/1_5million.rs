use driver::player::{Player, Strategy};
use driver::random_store::{Store, take_sequence};
use dsv_writer::{Encoder, NewLine, QuoteMode, ToDsv};
use implementation::lottery::Spec;
use implementation::prelude::*;
use rand::Rng;
use rayon::prelude::*;
use std::fs::File;

use rust_decimal::Decimal;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

fn main() {
	println!("1_5million Version: {}", env!("CARGO_PKG_VERSION"));
	println!("impl Version: {}", implementation::version());

	let _annual = annual();
	let _rng = rand::rng();

	let mut players = Vec::new();
	let agg = Mutex::new(Vec::new());

	for i in 0..750_000 {
		players.push(Player::new(i, Strategy::Sequence10));
	}

	for i in 750_000..1_500_000 {
		players.push(Player::new(i, Strategy::Random10));
	}

	let range = if cfg!(debug_assertions) { 0..5 } else { 0..60 };

	if cfg!(debug_assertions) {
		for year in range {
			for spec in annual() {
				let r = proc(year, &spec, &mut rand::rng(), &players);

				let mut lock = agg.lock().unwrap();
				lock.push((year, spec.name().to_string(), r));
			}
		}
	} else {
		range.into_par_iter().for_each(|year| {
			for spec in annual() {
				let r = proc(year, &spec, &mut rand::rng(), &players);

				let mut lock = agg.lock().unwrap();
				lock.push((year, spec.name().to_string(), r));
			}
		});
	}

	{
		let file = File::create("./15output_players.tsv").unwrap();
		let mut writer =
			dsv_writer::RawWriter::try_new(file, '\t', dsv_writer::NewLine::Lf).unwrap();

		writer.write_str_field("id", QuoteMode::AutoDetect).unwrap();
		writer
			.write_str_field("strategy", QuoteMode::AutoDetect)
			.unwrap();

		writer
			.write_str_field("prize_kind", QuoteMode::AutoDetect)
			.unwrap();
		writer
			.write_str_field("cnt", QuoteMode::AutoDetect)
			.unwrap();
		writer
			.write_str_field("payout", QuoteMode::AutoDetect)
			.unwrap();

		writer.end_of_record(true).unwrap();

		for p in players.iter() {
			p.to_dsv(&mut writer).unwrap();
		}
	}

	{
		let file = File::create("./15output_outcome.tsv").unwrap();
		let mut writer = dsv_writer::RawWriter::try_new(file, '\t', NewLine::Lf).unwrap();

		writer
			.write_str_field("year", QuoteMode::AutoDetect)
			.unwrap();
		writer
			.write_str_field("name", QuoteMode::AutoDetect)
			.unwrap();
		writer
			.write_str_field("prize_kind", QuoteMode::AutoDetect)
			.unwrap();
		writer
			.write_str_field("cnt", QuoteMode::AutoDetect)
			.unwrap();
		writer
			.write_str_field("payout", QuoteMode::AutoDetect)
			.unwrap();

		writer.end_of_record(false).unwrap();

		let lock = agg.lock().unwrap();

		for (yr, nme, arr) in lock.iter() {
			for (kind, cnt, reward) in arr.iter() {
				writer.write_value_field(yr, QuoteMode::AutoDetect).unwrap();
				writer.write_str_field(nme, QuoteMode::AutoDetect).unwrap();

				let p = match kind {
					PrizeCategory::First => "first",
					PrizeCategory::FirstAdjacent => "first_adjacent",
					PrizeCategory::FirstDifferentGroup => "first_different_group",
					PrizeCategory::Second => "second",
					PrizeCategory::Third => "third",
					PrizeCategory::Fourth => "fourth",
					PrizeCategory::Fifth => "fifth",
					PrizeCategory::Sixth => "sixth",
					PrizeCategory::Seventh => "seventh",
				};

				writer.write_str_field(&p, QuoteMode::AutoDetect).unwrap();
				writer
					.write_value_field(cnt, QuoteMode::AutoDetect)
					.unwrap();
				writer
					.write_value_field(reward, QuoteMode::AutoDetect)
					.unwrap();

				writer.end_of_record(false).unwrap();
			}
		}
	}
}

fn grp_100(
	spec: &Spec,
	rng: &mut impl Rng,
	players: &[Player],
) -> [(PrizeCategory, u32, Decimal); 9] {
	let shelf = Shelf::build_with_rand(1, 0..=100, rng);
	let rand_store = Store::new(
		10_000_000,
		&mut Shelf::build_with_rand(0, 0..=100, rng),
		rng,
	);

	lottery_proc(spec, rng, players, shelf, rand_store)
}

fn grp_200(
	spec: &Spec,
	rng: &mut impl Rng,
	players: &[Player],
) -> [(PrizeCategory, u32, Decimal); 9] {
	let mut shelf = Shelf::build_with_rand(0, 0..=200, rng);
	let rand_store = Store::new(10_000_000, &mut shelf, rng);

	lottery_proc(spec, rng, players, shelf, rand_store)
}

fn lottery_proc(
	spec: &Spec,
	rng: &mut impl Rng,
	players: &[Player],
	mut shelf: Shelf,
	mut rand_store: Store,
) -> [(PrizeCategory, u32, Decimal); 9] {
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

	#[cfg(debug_assertions)]
	{
		let mut min_grp = 10;
		let mut min_num = 10;
		for (t, p) in p.iter() {
			match p.strategy() {
				Strategy::Random10 => {
					let mut grp = HashSet::new();
					let mut num = HashSet::new();

					for (i, elem) in t.iter().enumerate() {
						grp.insert(elem.group());
						num.insert(elem.number() / 10);
						assert_eq!(elem.number() % 10, i as u32);
					}

					min_grp = usize::min(min_grp, grp.len());
					min_num = usize::min(min_num, num.len());

					assert!(grp.len() >= 5, "id:{}:{}", p.id(), grp.len());
					assert!(num.len() >= 5, "{}", num.len());
				}
				Strategy::Sequence10 => {
					let mut recent = t[0].number();
					let grp = t[0].group();

					for elem in t.iter() {
						assert_eq!(elem.number(), recent);
						assert_eq!(elem.group(), grp);
						recent += 1;
					}
				}
			}
		}

		println!("min_grp:{}", min_grp);
		println!("min_num:{}", min_num);
	}

	let outcome = drawing_machine::draw(spec, rng);

	let mut dict: HashMap<PrizeCategory, (u32, rust_decimal::Decimal)> = HashMap::new();

	for p in prizes() {
		dict.insert(p, (0, Decimal::ZERO));
	}

	for (tickets, player) in p.iter() {
		for t in tickets.iter() {
			let payout = outcome.verify(t);

			for elem in payout.records() {
				let (cnt, pay) = dict.get_mut(&elem.rank()).unwrap();
				*cnt += 1;
				*pay += elem.payout();
			}

			while !player.try_accumlate(&payout, spec.unit_price()) {
				std::hint::spin_loop();
			}
		}
	}

	prizes().map(|p| {
		let (c, pay) = dict.get(&p).unwrap();
		(p, *c, *pay)
	})
}

fn proc(
	year: u8,
	spec: &Spec,
	rng: &mut impl Rng,
	players: &[Player],
) -> [(PrizeCategory, u32, Decimal); 9] {
	println!("{}年 {}", year, spec.name());

	if spec.group_size() == 100 {
		grp_100(spec, rng, players)
	} else if spec.group_size() == 200 {
		grp_200(spec, rng, players)
	} else {
		unreachable!()
	}
}
