fn main(){
    let v = vec!["APS 1-2","APS 3-5","APS 5-8","EL 18-10","EL2 10-13","SES"];
    println!("What is your job?");
    let mut job = String::new();
    std::io::stdin().read_line(&mut job).expect("Failed to read input");

    println!("The experience you have will be determined by the serial numbers\n0.1-2 years\n1.3-5 years\n2.5-8 years\n3.8-10 years\n4.10-13 years\n5.above 13 years");
    println!("which of the following suites your experience?");

    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let index:usize = input1.trim().parse().expect("Failed to read input");
    let ch = v[index];

    println!("Your staff level is {}",ch);



}

