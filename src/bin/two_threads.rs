extern crate time;

use std::thread;
use std::sync::mpsc;

fn main() {

	let start = time::precise_time_ns();

	let (sender, receiver) = mpsc::channel();

	let precision: usize = 16;

	let task1 = sender.clone();
	thread::spawn(move || {
		for i in 0usize..precision {
			let k = i as f64;
			let p16: f64 = usize::pow(16, i as u32) as f64;
			let p = 1.0/p16 * (4.0/(8.0 * k + 1.0) - 2.0/(8.0 * k + 4.0) - 1.0/(8.0 * k + 5.0) - 1.0/(8.0 * k + 6.0));
			task1.send(p);
		}
	});

	let task2 = sender.clone();
	thread::spawn(move || {
		for i in 0usize..precision {
			let k = i as f64;
			let p16: f64 = usize::pow(16, i as u32) as f64;
			let p = 1.0/p16 * (4.0/(8.0 * k + 1.0) - 2.0/(8.0 * k + 4.0) - 1.0/(8.0 * k + 5.0) - 1.0/(8.0 * k + 6.0));
			task2.send(p);
		}
	});

	let mut pi: f64 = 0.0;
	for _ in 0usize..precision {
		pi += receiver.recv().unwrap();
	}

	println!("Pi approximation: {:.16}", pi);
	println!("Execution time:   {} ns", time::precise_time_ns() - start);

}
