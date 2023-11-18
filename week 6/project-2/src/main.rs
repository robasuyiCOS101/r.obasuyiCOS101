use std::io;

fn main(){
    let mut allowed_count = 0;
    while allowed_count <= 500 {

        println!("WELCOME TO THE RPIS");

        println!("Enter your name");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to read input");

        println!("Enter the number of papers published");
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Failed to read input");
        let num:i64 = num.trim().parse().expect("Failed to read input2");

        if num == 3 || num == 4 || num == 5 {
            println!("The incentive is N500_000 ");
        }else if num > 5 && num < 10{
            println!("The incentive is N800_000");
        }else if num >= 10{
            println!("The incentive is N1_000_000");
        } else{
            println!("the inccentive id N100_000");
        }allowed_count +=1;
    } 


}