fn main (){
    let mut name : Vec<String> = Vec::new();
    println!("The name vector has element {}",name.len());
    let mut experience: Vec<String> = Vec::new();
    println!("The experience vector has element {}",experience.len());

    
    println!("How many people are being interviewed?");
    let mut number_of_people = String::new();
    std::io::stdin().read_line(&mut number_of_people).expect("Failed to read input");
    let number_of_people:usize = number_of_people.trim().parse().expect("Invalid input");
    for count in 0..number_of_people 
    {

        
        let mut name = String::new();
        println!("Your name {}", count+1);
        std::io::stdin().read_line(&mut name).expect("Failed to read input");
        let new_name:String = name.trim().parse().expect("Invalid input");
        name.push(new_name);

        
        let mut experience = String::new();
        println!("Years of experience {}",count+1 );
        std::io::stdin().read_line(&mut experience).expect("Failed to read input ");
        let new_experience:usize= experience.trim().parse().expect("Failed to read input");
        experience.push(new_experience);
    

    }





}