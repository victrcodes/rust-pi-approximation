extern crate time;

use std::num;
use std::comm;

fn main() {

	let start = time::precise_time_ns();

	let (sender, receiver): (Sender<f64>, Receiver<f64>) = comm::channel();

	let precision: int = 16;

	for i in range(0, precision) {
		let task = sender.clone();
		spawn(proc() {
			let k = i as f64;
			let p16: f64 = num::pow(16i, i as uint) as f64;
			let p = 1.0/p16 * (4.0/(8.0 * k + 1.0) - 2.0/(8.0 * k + 4.0) - 1.0/(8.0 * k + 5.0) - 1.0/(8.0 * k + 6.0));
			task.send(p)
		});
	}

	let mut pi: f64 = 0.0;
	for _ in range(0, precision) {
		pi += receiver.recv();
	}

	println!("Pi approximation: {:.16}", pi);
	println!("Execution time:   {} ns", time::precise_time_ns() - start);

}