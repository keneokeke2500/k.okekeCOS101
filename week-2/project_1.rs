<<<<<<< HEAD
fn main() {
	let p:f64 = 520_000_000.0;
	let r: f64 = 10.0;
	let n : f64 = 5.0;

	//compound interest
	let a = p * (1.0 + r/100.0).powf(n);
	println!("Amount is {}", a );
	let cl = a-p;
	println!("Coumpound Interest is {}", cl );
}
=======
fn main() {
	let p:f64 = 520_000_000.0;
	let r: f64 = 10.0;
	let n : f64 = 5.0;

	//compound interest
	let a = p * (1.0 + r/100.0).powf(n);
	println!("Amount is {}", a );
	let cl = a-p;
	println!("Coumpound Interest is {}", cl );
}
>>>>>>> 8b40a5492641e45b761739540f4ae5c3125458d0
