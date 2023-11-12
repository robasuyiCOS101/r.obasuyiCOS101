// Rust program to display the menu for the food items available to take order

use std::io;
fn main(){
    
    println!("Welcome to Russell's Deluxe  kitchhen");


    println!("\nThe menu for today are as follows");
    println!("Poundo yam/Edinkaiko - N3,200 ");
    println!("Fried rice and chicken - N3,000");
    println!("Amala and Ewedu Soup - N2,500");
    println!("Eba and Egusi soup - N2,000");
    println!("white Rice and stew ");

     println!("\nGood day, Please what do you want?");

     let mut food = String::new();
     io::stdin().read_line(&mut food).expect("Failed to read input");

     println!("\nquantity of food");

    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).expect("Not a valid string");
    let mut quantity:f32 = quantity.trim().parse().expect("Not a valid number");

   
      println!("\nprice of the food");

     let mut price = String::new();
    io::stdin().read_line(&mut price).expect("Not a valid string");
    let mut price:f32 = price.trim().parse().expect("Not avalid number");

   


    let total = quantity * price;
    // let total price > 10000 = total_price * 0.05

    println!("\nThe total of the food  {} (naira)",total);

    if total > 10000.0
    {
       let discount_price = total - (total * 0.05);
        println!("The discount price {} ", discount_price);
    } 





   
}

