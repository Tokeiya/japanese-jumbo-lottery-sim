#![expect(unused_variables, reason = "Under construction")]
#![expect(unfulfilled_lint_expectations, reason = "Under construction")]

use implementation::prelude::*;
use std::sync::{
	Mutex, MutexGuard,
	atomic::{AtomicBool, Ordering},
};

use dashmap::DashMap;
use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Strategy {
	Random10,
	Sequence10,
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
			payed: Decimal::zero(),
			prizes: map,
		}
	}
}

pub struct Player {
	id: u64,
	strategy: Strategy,
	pub datum: Mutex<Datum>,
}

impl Player {
	pub fn new(id: u64, strategy: Strategy) -> Self {
		Self {
			id,
			strategy,
			datum: Mutex::new(Datum::default()),
		}
	}

	pub fn id(&self) -> u64 {
		self.id
	}

	pub fn strategy(&self) -> Strategy {
		self.strategy
	}
}
