fn main (){
    let mut name : Vec<String> = Vec::new();
    let mut experience: Vec<String> = Vec::new();
    


    
    println!("How many people are being interviewed?");
    let mut number_of_people = String::new();
    std::io::stdin().read_line(&mut number_of_people).expect("Failed to read input");
    let number_of_people:usize = number_of_people.trim().parse().expect("Invalid input");
    for count in 0..number_of_people 
    {

        
        let mut input1 = String::new();
        println!("Candidate Name {} ", count+1);
        std::io::stdin().read_line(&mut input1).expect("Failed to read input");
        let new_name:String = input1.trim().parse().expect("Invalid input");
        name.push(new_name);
        
        

        
        let mut input2 = String::new();
        println!("Years of experience for candidate in years {}",count+1 );
        std::io::stdin().read_line(&mut input2).expect("Failed to read input ");
        let new_experience:String = input2.trim().parse().expect("Failed to read input");
        experience.push(new_experience);
    

    }






}