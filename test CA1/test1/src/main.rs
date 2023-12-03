use std::io;


fn main() 
{
  let mut  _name = String::new();
  let mut _date_of_birth = String::new();
  let mut _email_address = String::new();
  let mut _phone_number = String::new();
  let mut _number_of_siblings = String::new();
  let mut _number_of_children = String::new();
  let mut  _medical_diagnostics = String::new();
  let mut _village_of_residence = String::new();
  let mut _age = String::new();

  println!("\nwhat is your name ?" );
  io::stdin().read_line(&mut _name).expect("Failed to read input");

  println!("\nWhen were you born ?");
  io::stdin().read_line(&mut _date_of_birth).expect("Failed to read input");

  println!("/nYour email address"  );
  io::stdin().read_line(&mut _email_address).expect("Failed to read input");

  println!("\nYour phone number");
   io::stdin().read_line(&mut _phone_number).expect("not a valid string");
  let _phone_number:i32 = _phone_number.trim().parse().expect("Not a valid number");
   println!("\nyour age? ");
   io::stdin().read_line(&mut _age).expect("not a valid string");
  let _age:i32 = _age.trim().parse().expect("Not a valid number");



   println!("\nHow many siblings do you have ?");
  io::stdin().read_line(&mut _number_of_siblings).expect("not a valid string");
  let _number_of_siblings:i32 = _number_of_siblings.trim().parse().expect("Not a valid number");

 println!("\nHow many children do you have ?");
 io::stdin().read_line(&mut _number_of_children).expect("not a valid string");
  let _number_of_children:i32 = _number_of_children.trim().parse().expect("Not a valid number");

 println!("\nWhat were you diagnosed of" );
  io::stdin().read_line(&mut _medical_diagnostics).expect("not a valid string");

  println!("\nWhich village do you stay ?");
   io::stdin().read_line(&mut _village_of_residence).expect("not a valid string");
    let _village_a= 1200000;
    let _village_b= 550000;
    let _village_c =1500000;
    let _village_d= 800000;
    let _village_e = 450000;

    let _disease_a = 6;
    let _disease_b = 7;
    let _disease_c = 8;
    let _disease_d = 9;
    let _disease_e = 10;

 
 if _age >= 50 &&   && _number_of_children > 4{}
 {
    let total = _village_a - (20 / 100 *1200000);
 } else if _age == 30  &&_number_of_siblings > 4{}
 {
    let total = _village_b - (5 / 100 * 55000);
 }else if _age > 40 && _number_of_children > 3;
 {
    let total = _village_c - (15 / 100 8 1500000);
 }

}








 



     
