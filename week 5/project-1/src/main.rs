//Rust program to find the roots of a quadratic equations 
use std::io;

fn main() 
{
   
   println!("Enter your value for a1 (in x):");
   let mut a1 = String::new();
   io::stdin().read_line(&mut a1).expect("Not a valid string");
   let a1:f32 = a1.trim().parse().expect("Not a valid number");


   println!("Enter your value  for b2 (in x ^2):" );
   let mut b2 = String::new();
   io::stdin().read_line(&mut b2).expect("Not a valid string");
    let b2:f32 = b2.trim().parse().expect("Not a valid number");

    println!("Enter value for c3");
    let mut c3 = String::new();
   io::stdin().read_line(&mut c3).expect("Not a valid string");
    let c3:f32 = c3.trim().parse().expect("Not a valid number");
    
    //formulae
    let discriminant:f32 = (b2 * b2) - (4.0 * a1 * c3);
     println!("discriminant is {}", discriminant);

     if discriminant > 0.0 
     {
        println!("Then there are two distinct roots");
     }
     else if discriminant  == 0.0
     {
        println!("Then there is exactly one real root");
     }
     else if discriminant  < 0.0 
     {
        println!("Then there are no real roots");
     }


}