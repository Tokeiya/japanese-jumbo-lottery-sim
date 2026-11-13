use std::sync::{
	Barrier,
	atomic::{AtomicBool, Ordering},
};
use std::thread;

const N: usize = 1_000_000;

// ここを両方 SeqCst に変更すると破綻しなくなる
const STORE_ORDER: Ordering = Ordering::Release;
const LOAD_ORDER: Ordering = Ordering::Acquire;

fn main() {
	let a = AtomicBool::new(false);
	let b = AtomicBool::new(false);

	let seen_a = AtomicBool::new(false);
	let seen_b = AtomicBool::new(false);

	// Worker 2本 + Main 1本
	let start = Barrier::new(3);
	let finish = Barrier::new(3);

	let mut broken = 0usize;

	thread::scope(|scope| {
		// Thread A
		scope.spawn(|| {
			for _ in 0..N {
				start.wait();

				a.store(true, STORE_ORDER);
				seen_a.store(b.load(LOAD_ORDER), Ordering::Relaxed);

				finish.wait();
			}
		});

		// Thread B
		scope.spawn(|| {
			for _ in 0..N {
				start.wait();

				b.store(true, STORE_ORDER);
				seen_b.store(a.load(LOAD_ORDER), Ordering::Relaxed);

				finish.wait();
			}
		});

		// Main: 実験の進行と結果の集計
		for _ in 0..N {
			a.store(false, Ordering::Relaxed);
			b.store(false, Ordering::Relaxed);

			start.wait();
			finish.wait();

			if !seen_a.load(Ordering::Relaxed) && !seen_b.load(Ordering::Relaxed) {
				broken += 1;
			}
		}
	});

	println!("試行回数: {N}");
	println!("破綻回数: {broken}");
	println!("破綻率: {:.6}%", broken as f64 / N as f64 * 100.0);
}
