use implementation::prelude::*;
use rand::Rng;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
pub struct Store(Vec<VecDeque<Ticket>>);

impl Store {
	pub fn new(size: u32, rng: &mut impl Rng) -> Self {
		if size % 10 != 0 {
			panic!("size must be multiple of 10")
		}

		todo!()
	}

	pub fn take_10() -> [Ticket; 10] {
		todo!()
	}
}
