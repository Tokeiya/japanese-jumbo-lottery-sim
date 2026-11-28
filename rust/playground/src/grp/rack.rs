use crate::grp::slot::Slot;
use rand::prelude::*;
use rand::seq::IndexedMutRandom;

pub struct Rack(Vec<Slot>);

impl Rack {
	pub fn new(group_size: u8) -> Self {
		Self((0..group_size).map(Slot::new).collect())
	}

	pub fn issue(&mut self, rng: &mut impl Rng) -> Option<u8> {
		
		loop{
			let idx=self.0.choose()
		}
		
		todo!()
	}

	pub fn is_empty(&self) -> bool {
		todo!()
	}
}
