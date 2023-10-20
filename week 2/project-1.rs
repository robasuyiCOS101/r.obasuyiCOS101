fn main() {
	let p:u64 = 520000000;
	let r = 10;
	let n = 5;
	// compound interest
	let a = p * (1 + (r /100)) * n;
	println!("Amount is {}",a);
	let ci = a - p;
	println!("Compound Interest is {}", ci);

}