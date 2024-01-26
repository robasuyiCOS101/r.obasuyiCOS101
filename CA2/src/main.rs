

use std::io::Write;
struct Details {
    name:String,
    date_founded:i32,
    shares:i32,
    liabilities:i32

}
impl Details{
fn leverage_percentage(&self)->f32{
    (((self.shares - self.liabilities) / self.shares) * 100) as f32

}
}

fn main(){
    
    let det1 = Details {
        name:String::from("cadb"),
        date_founded:1965,
        shares:15000000,
        liabilities:5500000
    };
    let det2 = Details {
        name:String::from("cham"),
        date_founded:1974,
        shares:25000000,
        liabilities:8000000

    };
    let det3 = Details {
        name:String::from("dang"),
        date_founded:1970,
        shares:18000000,
        liabilities:10000000
        
    };
     let det4 = Details {
        name:String::from("flour"),
        date_founded:1960,
        shares:32000000,
        liabilities:4000000
    }; 
    let det5 = Details {
        name:String::from("nest"),       
        date_founded:1961,
        shares:8000000,
        liabilities:1500000
    };
    let det6 = Details {
        name:String::from("unil"),
        date_founded:1923,
        shares:37000000,
        liabilities:11000000
    };
    let det7 = Details {
        name:String::from("hone"),
        date_founded:1906,
        shares:34000000,
        liabilities:9000000
    };
    let det8 = Details {
        name:String::from("nige"),
        date_founded:1946,
        shares:12000000,
        liabilities:12000000
    };
    let mut file = std::fs::File::create("COMPANIES DETAILS.txt").expect("create failed");
    file.write_all("\t\t\tFILE\n".as_bytes()).expect("write failed");
    file.write_all("Company Name\t\tDate Founded\t\tAssets\t\tliabilities\t\t%leverages\t\t5% of lev\n".as_bytes()).expect("write failed");

       let  mut username = String::new();
    println!("what is your companies user name ?");
    std::io::stdin().read_line(&mut username).expect("Failed to read input");
    let mut password = String::new();
    println!("What is your password for {}",username);
    std::io::stdin().read_line(&mut password).expect("Failed to read input");




     if password.trim() == "cadb123" || password.trim() == "cham123" || password.trim() == "dang123" || password.trim() == "flour123" || password.trim() == "nest123" || password.trim() =="unil123" || password.trim() == "hone123" || password.trim() == "nige123"{
        println!("Your have been granted access to the file");

 file.write_all(det1.name.as_bytes()).expect("write failed");
 file.write_all("\t\t\t".as_bytes()).expect("write failed");   
 file.write_all(det1.date_founded.to_string().as_bytes()).expect("write failed");
 file.write_all("\t\t\t".as_bytes()).expect("write failed");
 file.write_all(det1.shares.to_string().as_bytes()).expect("write failed");
 file.write_all("\t".as_bytes()).expect("write failed");
 file.write_all(det1.liabilities.to_string().as_bytes()).expect("write failed");
 if det1.shares > 20000000{
    file.write_all(det1.leverage_percentage().to_string().as_bytes()).expect("write failed");
    file.write_all("\t\t\t".as_bytes()).expect("write failed");
 } 
 if det1.liabilities < 10000000{
    file.write_all((det1.leverage_percentage()* 0.05) .to_string().as_bytes()).expect("write failed");
    file.write_all("\t\t\t\t\t".as_bytes()).expect("write failed");
 }
  file.write_all("\n".as_bytes()).expect("write failed");
  file.write_all(det2.name.as_bytes()).expect("write failed");
 file.write_all("\t\t\t".as_bytes()).expect("write failed");   
 file.write_all(det2.date_founded.to_string().as_bytes()).expect("write failed");
 file.write_all("\t\t\t".as_bytes()).expect("write failed");
 file.write_all(det2.shares.to_string().as_bytes()).expect("write failed");
 file.write_all("\t".as_bytes()).expect("write failed");
 file.write_all(det2.liabilities.to_string().as_bytes()).expect("write failed");
 if det2.shares > 20000000{
    file.write_all(det2.leverage_percentage().to_string().as_bytes()).expect("write failed");
    file.write_all("\t\t\t".as_bytes()).expect("write failed");
 } 
 if det2.liabilities < 10000000{
    file.write_all((det2.leverage_percentage()* 0.05) .to_string().as_bytes()).expect("write failed");
    file.write_all("\t\t\t\t\t".as_bytes()).expect("write failed");
 }
  file.write_all("\n".as_bytes()).expect("write failed");
  file.write_all(det3.name.as_bytes()).expect("write failed");
 file.write_all("\t\t\t".as_bytes()).expect("write failed");   
 file.write_all(det3.date_founded.to_string().as_bytes()).expect("write failed");
 file.write_all("\t\t\t".as_bytes()).expect("write failed");
 file.write_all(det3.shares.to_string().as_bytes()).expect("write failed");
 file.write_all("\t".as_bytes()).expect("write failed");
 file.write_all(det3.liabilities.to_string().as_bytes()).expect("write failed");
 if det3.shares > 20000000{
    file.write_all(det1.leverage_percentage().to_string().as_bytes()).expect("write failed");
    file.write_all("\t\t\t".as_bytes()).expect("write failed");
 } 
 if det3.liabilities < 10000000{
    file.write_all((det3.leverage_percentage()* 0.05) .to_string().as_bytes()).expect("write failed");
    file.write_all("\t\t\t\t\t".as_bytes()).expect("write failed");
 }
 file.write_all("\n".as_bytes()).expect("write failed");
 file.write_all(det4.name.as_bytes()).expect("write failed");
 file.write_all("\t\t\t".as_bytes()).expect("write failed");   
 file.write_all(det4.date_founded.to_string().as_bytes()).expect("write failed");
 file.write_all("\t\t\t".as_bytes()).expect("write failed");
 file.write_all(det4.shares.to_string().as_bytes()).expect("write failed");
 file.write_all("\t".as_bytes()).expect("write failed");
 file.write_all(det4.liabilities.to_string().as_bytes()).expect("write failed");
 if det4.shares > 20000000{
    file.write_all(det4.leverage_percentage().to_string().as_bytes()).expect("write failed");
    file.write_all("\t\t\t".as_bytes()).expect("write failed");
 } 
 if det4.liabilities < 10000000{
    file.write_all((det4.leverage_percentage()* 0.05) .to_string().as_bytes()).expect("write failed");
    file.write_all("\t\t\t\t\t".as_bytes()).expect("write failed");
 }
  file.write_all("\n".as_bytes()).expect("write failed");
  file.write_all(det5.name.as_bytes()).expect("write failed");
 file.write_all("\t\t\t".as_bytes()).expect("write failed");   
 file.write_all(det5.date_founded.to_string().as_bytes()).expect("write failed");
 file.write_all("\t\t\t".as_bytes()).expect("write failed");
 file.write_all(det5.shares.to_string().as_bytes()).expect("write failed");
 file.write_all("\t\t".as_bytes()).expect("write failed");
 file.write_all(det5.liabilities.to_string().as_bytes()).expect("write failed");
 if det5.shares > 20000000{
    file.write_all(det5.leverage_percentage().to_string().as_bytes()).expect("write failed");
    file.write_all("\t\t\t".as_bytes()).expect("write failed");
 } 
 if det5.liabilities < 10000000{
    file.write_all((det5.leverage_percentage()* 0.05) .to_string().as_bytes()).expect("write failed");
    file.write_all("\t\t\t\t\t".as_bytes()).expect("write failed");
 }

file.write_all("\n".as_bytes()).expect("write failed");
  file.write_all(det6.name.as_bytes()).expect("write failed");
 file.write_all("\t\t\t".as_bytes()).expect("write failed");   
 file.write_all(det6.date_founded.to_string().as_bytes()).expect("write failed");
 file.write_all("\t\t\t".as_bytes()).expect("write failed");
 file.write_all(det6.shares.to_string().as_bytes()).expect("write failed");
 file.write_all("\t".as_bytes()).expect("write failed");
 file.write_all(det6.liabilities.to_string().as_bytes()).expect("write failed");
 if det6.shares > 20000000{
    file.write_all(det1.leverage_percentage().to_string().as_bytes()).expect("write failed");
    file.write_all("\t\t\t".as_bytes()).expect("write failed");
 } 
 if det6.liabilities < 10000000{
    file.write_all((det1.leverage_percentage()* 0.05) .to_string().as_bytes()).expect("write failed");
    file.write_all("\t\t\t\t\t\n".as_bytes()).expect("write failed");
 }
 file.write_all("\n".as_bytes()).expect("write failed");
  file.write_all(det7.name.as_bytes()).expect("write failed");
 file.write_all("\t\t\t".as_bytes()).expect("write failed");   
 file.write_all(det7.date_founded.to_string().as_bytes()).expect("write failed");
 file.write_all("\t\t\t".as_bytes()).expect("write failed");
 file.write_all(det7.shares.to_string().as_bytes()).expect("write failed");
 file.write_all("\t".as_bytes()).expect("write failed");
 file.write_all(det7.liabilities.to_string().as_bytes()).expect("write failed");
 if det7.shares > 20000000{
    file.write_all(det1.leverage_percentage().to_string().as_bytes()).expect("write failed");
    file.write_all("\t\t\t".as_bytes()).expect("write failed");
 } 
 if det7.liabilities < 10000000{
    file.write_all((det7.leverage_percentage()* 0.05) .to_string().as_bytes()).expect("write failed");
    file.write_all("\t\t\t\t\t\n".as_bytes()).expect("write failed");
 }
 file.write_all("\n".as_bytes()).expect("write failed");
  file.write_all(det8.name.as_bytes()).expect("write failed");
 file.write_all("\t\t\t".as_bytes()).expect("write failed");   
 file.write_all(det8.date_founded.to_string().as_bytes()).expect("write failed");
 file.write_all("\t\t\t".as_bytes()).expect("write failed");
 file.write_all(det8.shares.to_string().as_bytes()).expect("write failed");
 file.write_all("\t".as_bytes()).expect("write failed");
 file.write_all(det8.liabilities.to_string().as_bytes()).expect("write failed");
 if det8.shares > 20000000{
    file.write_all(det1.leverage_percentage().to_string().as_bytes()).expect("write failed");
    file.write_all("\t\t\t".as_bytes()).expect("write failed");
 } 
 if det8.liabilities < 10000000{
    file.write_all((det1.leverage_percentage()* 0.05) .to_string().as_bytes()).expect("write failed");
    file.write_all("\t\t\t\t\t\n".as_bytes()).expect("write failed");
 }


    }
    else{
        println!("Sorry incorrect password and due to this you are unable to access this file");
    }



   

 
 
}    