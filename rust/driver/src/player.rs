use implementation::prelude::*;
use std::ops::Add;
use std::sync::Mutex;

use implementation::draw::Payout;
use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;

use dsv_writer::{Encoder, QuoteMode, ToDsv, ToDsvResult};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Strategy {
	Random,
	Sequence,
}

pub struct Datum {
	pub payed: Decimal,
	pub prizes: std::collections::HashMap<PrizeCategory, (Decimal, usize)>,
}

impl Default for Datum {
	fn default() -> Self {
		let mut map = std::collections::HashMap::new();

		for p in prizes() {
			map.insert(p, (Decimal::zero(), 0));
		}

		Self {
			payed: Decimal::ZERO,
			prizes: map,
		}
	}
}

pub struct Player {
	id: u64,
	process_id: u32,
	strategy: Strategy,
	pub datum: Mutex<Datum>,
}

impl Player {
	pub fn new(process_id: u32, id: u64, strategy: Strategy) -> Self {
		Self {
			id,
			process_id,
			strategy,
			datum: Mutex::new(Datum::default()),
		}
	}

	pub fn id(&self) -> u64 {
		self.id
	}

	pub fn process_id(&self) -> u32 {
		self.process_id
	}

	pub fn strategy(&self) -> Strategy {
		self.strategy
	}

	pub fn try_accumulate(&self, payout: &Payout, cost: Decimal) -> bool {
		if let Ok(mut d) = self.datum.try_lock() {
			d.payed = d.payed.add(cost);

			for p in payout.records().iter() {
				let (ttl, cnt) = d.prizes.get_mut(&p.rank()).unwrap();

				*ttl = ttl.add(p.payout());
				*cnt += 1;
			}

			true
		} else {
			false
		}
	}
}

impl ToDsv for Player {
	type Error = std::io::Error;

	fn to_dsv<T: Encoder>(&self, writer: &mut T) -> ToDsvResult<(), Self::Error> {
		let tmp = match self.strategy {
			Strategy::Random => "random",
			Strategy::Sequence => "sequence",
		};

		let lock = self.datum.lock().unwrap();

		for p in prizes() {
			let (pay, cnt) = lock.prizes.get(&p).unwrap();

			writer.write_value_field(&self.id, QuoteMode::AutoDetect)?;
			writer.write_str_field(tmp, QuoteMode::AutoDetect)?;

			let p = match &p {
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

			writer.write_str_field(p, QuoteMode::AutoDetect)?;
			writer.write_value_field(cnt, QuoteMode::AutoDetect)?;
			writer.write_value_field(pay, QuoteMode::AutoDetect)?;

			writer.end_of_record(true)?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn new() {
		let mut fixture = Player::new(100, 42, Strategy::Random);
		assert_eq!(fixture.id, 42);
		assert_eq!(fixture.strategy, Strategy::Random);
		assert_eq!(fixture.process_id, 100);

		let ptr = fixture.datum.get_mut().unwrap();
		assert_eq!(ptr.payed, Decimal::ZERO);

		for p in prizes() {
			let (accum, cnt) = ptr.prizes.get(&p).unwrap();
			assert_eq!(*accum, Decimal::ZERO);
			assert_eq!(*cnt, 0)
		}
	}

	#[test]
	fn process_id() {
		let fixture = Player::new(100, 42, Strategy::Random);
		assert_eq!(100, fixture.process_id);
	}

	#[test]
	fn id() {
		let fixture = Player::new(100, 42, Strategy::Random);
		assert_eq!(fixture.id(), 42);
	}

	#[test]
	fn strategy() {
		let fixture = Player::new(100, 42, Strategy::Random);
		assert_eq!(fixture.strategy(), Strategy::Random);
	}

	#[test]
	fn try_accumlate() {
		use implementation::prelude::payout::Record;
		let fixture = Player::new(100, 42, Strategy::Random);

		let pay = Payout::new(
			Ticket::new(2, 10, 114_514),
			vec![
				Record::new(PrizeCategory::First, Decimal::from(1_000_000)),
				Record::new(PrizeCategory::FirstAdjacent, Decimal::from(100_000)),
			],
		);

		let p = fixture.datum.lock().unwrap();

		assert!(!fixture.try_accumulate(&pay, Decimal::from(3_000)));

		drop(p);

		assert!(fixture.try_accumulate(&pay, Decimal::from(3_000)));

		let lock = fixture.datum.lock().unwrap();

		for p in prizes() {
			if p == PrizeCategory::First {
				let (d, u) = lock.prizes.get(&PrizeCategory::First).unwrap();
				assert_eq!(d, &Decimal::from(1_000_000));
				assert_eq!(u, &1)
			} else if p == PrizeCategory::FirstAdjacent {
				let (d, u) = lock.prizes.get(&PrizeCategory::FirstAdjacent).unwrap();
				assert_eq!(d, &Decimal::from(100_000));
				assert_eq!(u, &1);
			} else {
				let (d, u) = lock.prizes.get(&p).unwrap();
				assert_eq!(d, &Decimal::ZERO);
				assert_eq!(u, &0);
			}
		}
	}
}
