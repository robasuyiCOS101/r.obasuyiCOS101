use std::io;


fn main(){
    println!("What timestable would you like ? ");

    let mut n = String ::new();
    io::stdin().read_line(&mut n).expect("Failed to read input");
    let n:i64 = n.trim().parse().expect("Not a valid number");

    for x in 1..12
    {
        let t:i64 = n * x;
        println!("{} X {} = {}",n,x,t);
 
    }

  


}