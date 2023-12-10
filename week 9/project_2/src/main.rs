 
 use std::io::Write;
fn main(){

    let mut file = std::fs::File::create("PAU_SMIS.txt").expect("create failed");
  file.write_all("\t\t\t\tPAU SMIS FILE\n".as_bytes()).expect("write failed");
  file.write_all("student_name\t\t\tmatric_number\t\t\tdepartment\t\tlevel\n".as_bytes());
  

    let mut student_name: Vec<String> = Vec::new();
    let mut matric_number: Vec<String> = Vec::new();
    let mut department: Vec<String> = Vec::new();
    let mut level: Vec<String> = Vec::new();

    println!("How many Students details are needed?");
    let mut number_of_students = String::new();
    std::io::stdin().read_line(&mut number_of_students).expect("Failed to read input");
    let number_of_students:i32 = number_of_students.trim().parse().expect("Failed to read input");
    

    for count in 0..number_of_students {

        println!("What is the student's name {} ?", count+1);   
        let  mut input1 = String::new(); 
        std::io::stdin().read_line(&mut input1).expect("Failed to read input");
        let new_student_name:String = input1.trim().parse().expect("Invalid input");
        student_name.push(new_student_name.to_string());
        file.write_all(new_student_name.as_bytes()).expect("create failed");
        file.write_all("\t\t\t".as_bytes()).expect("create failed");

        let mut input2 = String::new();
        println!("What is the student's matric number {} ?", count+1);
        std::io::stdin().read_line(&mut input2).expect("Failed to read input ");
        let new_matric_number:String = input2.trim().parse().expect("Failed to read input");
        matric_number.push(new_matric_number.to_string());
        file.write_all(new_matric_number.as_bytes()).expect("create failed");
        file.write_all("\t\t\t".as_bytes()).expect("create failed");

        let mut input3 = String::new();
        println!("What is the student's Department {} ?", count+1);
        std::io::stdin().read_line(&mut input3).expect("Failed to read input ");
        let new_department:String = input3.trim().parse().expect("Failed to read input");
        department.push(new_department.to_string());
        file.write_all(new_department.as_bytes()).expect("create failed");
        file.write_all("\t\t\t".as_bytes()).expect("create failed");


        let mut input4 = String::new();
        println!("What is the student's level {} ?", count+1);
        std::io::stdin().read_line(&mut input4).expect("Failed to read input ");
        let new_level:String = input4.trim().parse().expect("Failed to read input");
        level.push(new_level.to_string());
        file.write_all(new_level.as_bytes()).expect("create failed");
        file.write_all("\t\t\t".as_bytes()).expect("create failed");

    }

    

  
}
