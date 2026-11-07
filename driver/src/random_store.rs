use implementation::prelude::*;
use rand::Rng;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
pub struct Store(Vec<VecDeque<Ticket>>);

impl Store {
	pub fn new(
		shelf: &mut shelf::Shelf,
		unit_size: u32,
		group_size: u8,
		rng: &mut impl Rng,
	) -> Self {
		let mut vec = (0..group_size)
			.map(|_| VecDeque::<Ticket>::new())
			.collect::<Vec<VecDeque<_>>>();

		for _ in 0..unit_size {}

		todo!()
	}

	pub fn random_10(&mut self) -> Vec<Ticket> {
		todo!()
	}
}
