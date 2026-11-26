use dsv_writer::{Encoder, NewLine, QuoteMode, RawWriter, Result};
use implementation::outcome::Record;
use implementation::prelude::*;
use std::fs::File;

fn main() {
	let mut rng = rand::rng();

	let mut writer =
		RawWriter::try_new(File::create_new("outcome.tsv").unwrap(), '\t', NewLine::Lf).unwrap();

	writer
		.write_str_field("year", QuoteMode::AutoDetect)
		.unwrap();
	writer
		.write_str_field("name", QuoteMode::AutoDetect)
		.unwrap();

	writer
		.write_str_field("prize_tier", QuoteMode::AutoDetect)
		.unwrap();

	writer
		.write_str_field("group", QuoteMode::AutoDetect)
		.unwrap();
	writer
		.write_str_field("number", QuoteMode::AutoDetect)
		.unwrap();
	writer.end_of_record(false).unwrap();

	for year in 0..1_000 {
		for spec in annual() {
			dump(
				&mut writer,
				year,
				spec.name(),
				drawing_machine::draw(&spec, &mut rng).records(),
			)
			.unwrap();
		}
	}
}

fn dump(writer: &mut RawWriter<File>, year: u16, name: &str, records: &[Record]) -> Result<()> {
	for elem in records.iter() {
		writer.write_value_field(&year, QuoteMode::AutoDetect)?;
		writer.write_str_field(name, QuoteMode::AutoDetect)?;
		let prize = match elem.rank() {
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
		writer.write_str_field(prize, QuoteMode::AutoDetect)?;

		match elem.group().value() {
			Some(v) => writer.write_value_field(&v, QuoteMode::AutoDetect)?,
			None => writer.write_str_field("", QuoteMode::AutoDetect)?,
		};

		writer.write_value_field(&elem.number().value(), QuoteMode::AutoDetect)?;

		writer.end_of_record(false)?;
	}

	Ok(())
}
