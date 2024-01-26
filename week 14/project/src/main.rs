use std::io;
use std::io::Read;

fn adminstrator(){
    println!("You have been granted access to the file");
    let mut file = std::fs::File::open("globacom_dbase.sql");
        let mut db_struct = String::new();
        file.expect("REASON").read_to_string(&mut db_struct).unwrap();
        print!("{}",db_struct);

}
  
  fn project_manager(){
     println!("You have been granted access to the file");
          let mut file = std::fs::File::open("PROJECT_tb.sql");
        let mut project = String::new();
        file.expect("REASON").read_to_string(&mut project).unwrap();
        print!("{}",project);
 }

  fn employee(){
     println!("You have been granted access to the file");
       let mut file = std::fs::File::open("STAFF_tb.sql");
        let mut staff_struct = String::new();
        file.expect("REASON").read_to_string(&mut staff_struct).unwrap();
        print!("{}",staff_struct);
  }


   fn customer(){
    println!("You have been granted access to the file");
    let mut file = std::fs::File::open("CUSTOMER_tb.sql");
        let mut customer_struct = String::new();
        file.expect("REASON").read_to_string(&mut customer_struct).unwrap();
        print!("{}",customer_struct);
   }

   fn vendor(){
    println!("You have been granted access to the file");
      let mut file = std::fs::File::open("DATAPLAN_tb.sql");
        let mut dataplan = String::new();
        file.expect("REASON").read_to_string(&mut dataplan).unwrap();
        print!("{}",dataplan);
   }

   fn main(){
    println!("YOU ARE WELCOME TO GLOBACOM Ltd DATABASE SYSTEM");
    println!("Your position?\n Hint: an adminstrator, a project manager, an employee, a customer or a vendor");
    let  mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read inpput");
    let position = input.trim();

    if position == "adminstrator" {
        adminstrator();
    }
    else if position == "project manager" {
        project_manager();
    }
    else if position == "employee" {
        employee();
    }
    else if position =="customer" {
        customer();
    }
    else if position == "vendor" {
        vendor();
    }
   }
