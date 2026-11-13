use overwrite_ring_buffer::CircularBuffer;
use overwrite_ring_buffer::fixed::*;
use std::hash::Hasher;

fn main() {}

pub struct MyHasher(u64, Buffer<u8, GeneralIndexCoordinator<9>, 9>, usize);

impl MyHasher {
	fn hash_proc(&mut self) {
		todo!()
	}
}
impl Default for MyHasher {
	fn default() -> Self {
		let mut buff = Buffer::default();

		for i in 0..buff.capacity() {
			buff.enqueue(0)
		}

		Self(0, buff, 0)
	}
}

impl Hasher for MyHasher {
	fn finish(&self) -> u64 {
		todo!()
	}

	fn write(&mut self, bytes: &[u8]) {
		for elem in bytes.iter() {
			self.1.enqueue(*elem);
			self.2 += 1;

			if self.2 == 9 {}
		}
	}
}
