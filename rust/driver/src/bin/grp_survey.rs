use clap::Parser;
use driver::args::Args;
use driver::random_store::*;
use dsv_writer::{Encoder, NewLine, QuoteMode, RawWriter};
use implementation::prelude::*;
use rayon::prelude::*;
use std::fs::File;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::*;
use std::thread::{self, JoinHandle};

struct AsyncWriter {
	tx: Option<SyncSender<(usize, [usize; 10])>>,
	handle: Option<JoinHandle<()>>,
}

const ITERATION: usize = 10_000;

impl AsyncWriter {
	fn thread_proc(process_id: u32, file: File, rx: Receiver<(usize, [usize; 10])>) {
		let mut writer = RawWriter::try_new(file, '\t', NewLine::Lf).unwrap();

		writer.write_str_field("id", QuoteMode::AutoDetect).unwrap();
		writer
			.write_str_field("process_id", QuoteMode::AutoDetect)
			.unwrap();
		writer
			.write_str_field("overlap", QuoteMode::AutoDetect)
			.unwrap();
		writer
			.write_str_field("count", QuoteMode::AutoDetect)
			.unwrap();

		writer.end_of_record(false).unwrap();

		for (id, arr) in rx.iter() {
			for (i, cnt) in arr.iter().enumerate() {
				writer
					.write_value_field(&id, QuoteMode::AutoDetect)
					.unwrap();
				writer
					.write_value_field(&(process_id + 1), QuoteMode::AutoDetect)
					.unwrap();
				writer.write_value_field(&i, QuoteMode::AutoDetect).unwrap();
				writer
					.write_value_field(cnt, QuoteMode::AutoDetect)
					.unwrap();

				writer.end_of_record(false).unwrap();
			}
		}
	}
	pub fn new(process_id: u32, file: File, threshold: usize) -> Self {
		let (tx, rx) = sync_channel::<(usize, [usize; 10])>(threshold);
		let handle = thread::spawn(move || Self::thread_proc(process_id, file, rx));
		Self {
			tx: Some(tx),
			handle: Some(handle),
		}
	}

	fn write(&self, id: usize, datum: [usize; 10]) {
		self.tx.as_ref().unwrap().send((id, datum)).unwrap();
	}

	fn wait(&mut self) {
		let tmp = self.tx.take();
		drop(tmp);

		self.handle.take().unwrap().join().unwrap();
	}
}
fn main() {
	let arg = Args::parse();

	let file = File::create("grp_100.tsv").unwrap();
	let mut async_writer = AsyncWriter::new(arg.process_id, file, 100);
	let cnt = AtomicUsize::new(0);

	(1..=ITERATION).into_par_iter().for_each(|i| {
		let ret = proc_100(42);
		async_writer.write(i, ret);
		let c = cnt.fetch_add(1, Ordering::Relaxed);
		if c & 0x7f == 0 {
			println!(
				"{c}/{ITERATION} {:.2}",
				(c as f64) / (ITERATION as f64) * 100.0
			);
		}
	});

	async_writer.wait();
}

fn accumulate_tickets(tickets: &[Ticket], accumulator: &mut [usize; 10]) {
	let mut agg = [0usize; 200];

	for t in tickets.iter() {
		agg[(t.group() - 1) as usize] += 1;
	}

	for n in agg.iter().filter(|x| x != &&0) {
		accumulator[*n - 1] += 1;
	}
}

#[allow(dead_code)]
fn proc_200(unit: u8) -> [usize; 10] {
	let mut rng = rand::rng();

	let mut shelf = Shelf::build_with_rand(unit, 1..=200, &mut rng);
	let mut store = Store::new(7_500_000, &mut shelf, &mut rng);
	drop(shelf);

	let mut accumulator = [0usize; 10];

	for _ in 0..7_500_000 {
		let tickets = store.take_10().unwrap();
		accumulate_tickets(&tickets, &mut accumulator);
	}

	accumulator
}

#[allow(dead_code)]
fn proc_100(unit: u8) -> [usize; 10] {
	let mut rng = rand::rng();

	let mut shelf = Shelf::build_with_rand(unit, 1..=100, &mut rng);
	let mut store = Store::new(7_500_000, &mut shelf, &mut rng);

	drop(shelf);

	let mut accumulator = [0usize; 10];

	for _ in 0..750_000 {
		let tickets = store.take_10().unwrap();
		accumulate_tickets(&tickets, &mut accumulator);
	}

	accumulator
}
