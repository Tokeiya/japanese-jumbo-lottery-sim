use super::player::Player;
use dsv_writer::{Encoder, NewLine, QuoteMode, ToDsv};
use implementation::prelude::*;
use rust_decimal::Decimal;
use std::fs::File;
use std::path::Path;

type AggDatum = (PrizeCategory, u32, Decimal);
pub fn dump_players(path: &Path, scr: &[Player]) -> std::io::Result<()> {
	let file = File::create(path)?;
	let mut writer = dsv_writer::RawWriter::try_new(file, '\t', dsv_writer::NewLine::Lf).unwrap();

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

	for p in scr.iter() {
		p.to_dsv(&mut writer).unwrap();
	}

	Ok(())
}

pub fn dump_outcome(path: &Path, scr: &[(u8, String, [AggDatum; 9])]) -> std::io::Result<()> {
	let file = File::create(path)?;
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

	for (yr, nme, arr) in scr.iter() {
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

	Ok(())
}
