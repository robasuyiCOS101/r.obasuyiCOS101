
use std::io;

fn main () 
{
    let mut input1 = String::new();
    println!("\nEnter your age (in years)");
    io::stdin().read_line(&mut input1).expect("Not a valid String");
    let age:i32 = input1.trim().parse().expect("'Nota valid number");

    let mut input2 = String::new();
    println!("true if experience and false if inexperienced");
    let experience =  "true";
    io::stdin().read_line(&mut input2).expect("Not a valid String");
    
    
    if  experience == "true"  && age >= 40 
    {
        println!(" the incentive of the employee is 1560000 (in naira)");
    }
    else if experience == "true"  && age >= 30 && age <40
    {
        println!("the incentive of the employee is 1480000 (in naira) ");
    } 
    else
    {
        println!("the incentive will be 100000  (in naira)");
    }

    
}