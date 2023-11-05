use std::io;

fn main(){
    let mut distance1= String::new();
    let mut time1 = String::new();
    let mut distance2 =String::new();
    let mut time2 = String::new();
    let mut speed1 = String::new();
    let mut speed2 = String::new();
    let  a1:f32 = 80.0;
    let  a2:f32 = 120.0;
    println!("Enter value for distance1 {}", a1);
    io::stdin().read_line(&mut distance1).expect("Not a valid string");
    let d:f32 = a1 * 1.609;   //converting to kilometres
    let d:f32 = distance1.trim().parse().expect("Not a valid number");
    println!("distance1 is {} km", d);

    println!("Enter value for time1 {}", time1);
    io::stdin().read_line(&mut time1).expect("Not a valid string");
    let t:f32 = 2.0;
    let t:f32 = time1.trim().parse().expect("Not a valid string");
    println!("time1 is in {} h", t);

    let mut speed1:f32 = d / t;
     println!("speed of first car {} km/h", speed1);
 
    // if it goes 120miles in 4hours
     println!("Enter value for distance2 {}", a2);
    let x:f32 = a2 * 1.069;
    io::stdin().read_line(&mut distance2).expect("Not a valid string");
    let x:f32 = distance2.trim().parse().expect("Not a valid number");
     println!("distance2 is {} km", x);

   println!("Enter value for time2 {} ", time2);
    io::stdin().read_line(&mut time2).expect("Not a valid string");
    let y:f32 = 4.0;
    let y:f32 = time2.trim().parse().expect("Not a valid string");
     println!("time2 is in {} s", y);

    let mut speed2:f32 = x / y;

  println!("speed2 of the second car is  {} km/h", speed2)  


}