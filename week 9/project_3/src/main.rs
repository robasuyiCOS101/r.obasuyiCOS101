 
use std::io::Write;
fn main(){

    let mut file = std::fs::File::create("MINISTERS LIST.txt").expect("create failed");
  file.write_all("\t\t\t\tFILE\n".as_bytes()).expect("write failed");
  file.write_all("name of commissioner\t\t\tministry\t\t\tgeopolitical zone\n".as_bytes());
  

    let mut name_of_commissioner: Vec<String> = Vec::new();
    let mut ministry: Vec<String> = Vec::new();
    let mut geopolitical_zone: Vec<String> = Vec::new();
    

    println!("How many commisioners are there?");
    let mut number_of_commissioners = String::new();
    std::io::stdin().read_line(&mut number_of_commissioners).expect("Failed to read input");
    let number_of_commissioners:i32 = number_of_commissioners.trim().parse().expect("Failed to read input");
    

    for count in 0..number_of_commissioners {

        println!("What is the commissioner's name {} ?", count+1);   
        let  mut input1 = String::new(); 
        std::io::stdin().read_line(&mut input1).expect("Failed to read input");
        let new_commissioner_name:String = input1.trim().parse().expect("Invalid input");
        name_of_commissioner.push(new_commissioner_name.to_string());
        file.write_all(new_commissioner_name.as_bytes()).expect("create failed");
        file.write_all("\t\t\t".as_bytes()).expect("create failed");

        let mut input2 = String::new();
        println!("What ministry is the commissioner in {} ?", count+1);
        std::io::stdin().read_line(&mut input2).expect("Failed to read input ");
        let new_ministry:String = input2.trim().parse().expect("Failed to read input");
        ministry.push(new_ministry.to_string());
        file.write_all(new_ministry.as_bytes()).expect("create failed");
        file.write_all("\t\t\t".as_bytes()).expect("create failed");

        let mut input3 = String::new();
        println!("What is the commissioner's geopolitical zone {} ?", count+1);
        std::io::stdin().read_line(&mut input3).expect("Failed to read input ");
        let new_geopolitical_zone:String = input3.trim().parse().expect("Failed to read input");
        geopolitical_zone.push(new_geopolitical_zone.to_string());
        file.write_all(new_geopolitical_zone.as_bytes()).expect("create failed");
        file.write_all("\n ".as_bytes()).expect("create failed");


       

    }

    

  
}
