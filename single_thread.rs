extern crate time;

use std::num;

fn main() {

	let start = time::precise_time_ns();

	let mut pi: f64 = 0.0;
	let precision: int = 16;

	for i in range(0, precision) {
		let k = i as f64;
		let p16: f64 = num::pow(16i, k as uint) as f64;
		pi += 1.0/p16 * (4.0/(8.0 * k + 1.0) - 2.0/(8.0 * k + 4.0) - 1.0/(8.0 * k + 5.0) - 1.0/(8.0 * k + 6.0));
	}

	println!("Pi approximation: {:.16}", pi);
	println!("Execution time:   {} ns", time::precise_time_ns() - start);

}