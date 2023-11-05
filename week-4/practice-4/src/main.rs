fn main() {
    let fullname = "Chibundum John Umeh";
    let department = "Computer science";
    let uni = "Pan-Atlantic University";


    let mut school = "School of Science".to_string();
    // push string
    school.push_str(" and technology");


    println!("HMy name is: {}", fullname);
    //check length
    println!("The length my full name is: {}",fullname.len());
    println!("I am a student of {} Department", department);
    println!("{}",school);
    println!("{}",uni);
}