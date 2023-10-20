fn main () {
	let t = 2 * 450000;
	let m:u64= 1 * 1500000;
	let h = 3 * 750000;
	let d:u64 = 3 * 2850000;
	let a = 1 * 250000;

    // sum
    let s = t + m + h + d + a;
    println!("Sum is {}",s);

    // average
    let y = s/10;
    println!("Average is {}",y);
}