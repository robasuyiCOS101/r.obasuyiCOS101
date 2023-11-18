use std::io;

fn main(){

    let mut elligible_count = 0;
    while elligible_count < 150 {

    println!("Welcome to the Student Council Voter System (SCVO)");

     println!("Are you A course rep ? (true/false) ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let course_rep:bool = input1.trim().parse().expect("Not Valid"); 

   

      println!("Enter your level: ");
    let mut  input2  =  String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let lvl:i32 = input2.trim().parse().expect("Not a Valid Number");
  
     println!("What is your CGPA ?" );

    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read input");
    let num:f32 = num.trim().parse().expect("Failed to read input");

   
    println!("Enter your Name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    println!("Enter your email: ");
    let mut email = String::new();
    io::stdin().read_line(&mut email).expect("Failed to read input");


    println!("Enter your department: ");
    let mut department = String::new();
    io::stdin().read_line(&mut department).expect("Failed to read input");

    println!("Enter your state_of_origin: ");
    let mut state_of_origin = String::new();
    io::stdin().read_line(&mut state_of_origin).expect("Failed to read input");

     if num > 4.0 && course_rep == false && lvl == 200 || lvl == 300 || lvl == 400 {
        println!("Name:{}", name);
        println!( "email: {}", email);
        println!("department: {}", department);
        println!("State of origin: {}", state_of_origin);
        println!("You can vote");     
    elligible_count +=1;
    }else {
        println!("Sorry,You are not elligible to vote");
    }

}    

                                        }