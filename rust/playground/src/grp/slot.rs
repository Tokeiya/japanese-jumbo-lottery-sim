const SIZE: usize = 100_000;

pub struct Slot(usize, u8);

impl Slot {
	pub fn new(group: u8) -> Self {
		Self(0, group)
	}
	pub fn issue(&mut self) -> Option<u8> {
		if self.0 >= SIZE {
			None
		} else {
			self.0 += 1;
			Some(self.1)
		}
	}

	pub fn issued_count(&self) -> usize {
		self.0
	}

	pub fn available_count(&self) -> usize {
		SIZE - self.0
	}

	pub fn group(&self) -> u8 {
		self.1
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new() {
		for g in 0..u8::MAX {
			let fixture = Slot::new(g);
			assert_eq!(fixture.0, 0);
			assert_eq!(fixture.1, g);
		}
	}

	#[test]
	fn issue() {
		let mut fixture = Slot(0, 42);

		let mut cnt = 0usize;

		while fixture.issue().is_some() {
			cnt += 1;
		}

		for _ in 0..1_000 {
			assert!(fixture.issue().is_none());
		}

		println!("{cnt}");
	}

	#[test]
	fn issued_count() {
		let mut fixture = Slot(0, 42);

		for i in 0..SIZE {
			assert_eq!(fixture.issued_count(), i);
			fixture.0 += 1;
		}

		assert_eq!(fixture.issued_count(), SIZE);
	}

	#[test]
	fn available_count() {
		let mut fixture = Slot(0, 42);

		for i in (1..=SIZE).rev() {
			assert_eq!(fixture.available_count(), i);
			fixture.0 += 1;
		}

		assert_eq!(fixture.available_count(), 0);
	}

	#[test]
	fn group() {
		for g in 0..=u8::MAX {
			let fixture = Slot::new(g);
			assert_eq!(fixture.group(), g);
		}
	}
}
