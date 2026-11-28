use driver::random_store::*;
use dsv_writer::{Encoder, NewLine, QuoteMode, RawWriter};
use implementation::prelude::*;
use rayon::prelude::*;
use std::fs::File;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::*;
use std::thread::{self, JoinHandle};
struct AsyncWriter {
	tx: SyncSender<(usize, [usize; 10])>,
	handle: JoinHandle<()>,
}

impl AsyncWriter {
	fn thread_proc(file: File, rx: Receiver<(usize, [usize; 10])>) {
		let mut writer = RawWriter::try_new(file, '\t', NewLine::Lf).unwrap();

		writer.write_str_field("id", QuoteMode::AutoDetect).unwrap();
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
				writer.write_value_field(&i, QuoteMode::AutoDetect).unwrap();
				writer
					.write_value_field(cnt, QuoteMode::AutoDetect)
					.unwrap();
			}
		}
	}
	pub fn new(file: File, threshold: usize) -> Self {
		let (tx, rx) = sync_channel::<(usize, [usize; 10])>(threshold);
		let handle = thread::spawn(move || Self::thread_proc(file, rx));
		Self { tx, handle }
	}

	fn write(&self, id: usize, datum: [usize; 10]) {
		self.tx.send((id, datum)).unwrap();
	}
}
fn main() {
	let file = File::create("grp_100.tsv").unwrap();
	let async_writer = AsyncWriter::new(file, 100);
	let cnt = AtomicUsize::new(0);

	(1..=1_000usize).into_par_iter().for_each(|i| {
		let ret = proc_100(42);
		async_writer.write(i, ret);
		let c = cnt.fetch_add(1, Ordering::Relaxed);
		if c & 0x7f == 0 {
			println!("{c}/1_000 {:.2}", (c as f64) / 1_000.0 * 100.0);
		}
	})
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
