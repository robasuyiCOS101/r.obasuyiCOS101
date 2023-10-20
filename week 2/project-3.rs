fn main() {
	let p = 210000;
	let r = 5;
	let n = 3;


	// compound interest
	let a = p *(1 - r/100) ^ n;
	println!("Amount is {}",a);
	let ci = a - p;
	println!("Compound Interest is {}",ci);


}