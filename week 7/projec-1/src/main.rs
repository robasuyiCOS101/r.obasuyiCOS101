use std::io;

fn main(){
    println!("What do you want to solve ?\nNote:The equation you want will be determined by the numbers  in bracket so if you want a particular equation type the number in the bracket \nArea of a Trapezium(1) \nArea of Rhombus(2) \nArea of Parallelogram(3)\nArea of Cube(4) \nVolume of Cylinder(5)");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice:i32 = choice.trim().parse().expect("Failed to input");

    if choice == 1 {
         area_of_trapezium(); 
    }else if  choice == 2{
         area_of_rhombus();
    }else if  choice == 3 {
          area_of_parallelogram();
    }else if choice == 4{
        area_of_cube();
    }else if choice == 5{
         volume_of_cylinder();
    }else {
        println!("Choose a number from 1 to 5");
    }

}

fn area_of_trapezium()->i32{
    println!("What is the height in cm ?");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let height:i32 = input1.trim().parse().expect("Failed to read input");

    println!("What is the first base in cm ? ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let base1:i32 = input2.trim().parse().expect("Failed to read input");

    println!("What is the second base ?");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let base2:i32 = input3.trim().parse().expect("Failed to read input");
    
    let t = height / 2 * (base1 + base2 );
    println!("The answer is {} cm squared",t);
    return t;

}

fn area_of_rhombus()->i32{
    println!("What is the first diagonal in cm ?");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let diagonal1:i32 = input1.trim().parse().expect("Failed to read input ");
    
    println!("What is the second diagonal in cm ?");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let diagonal2:i32 = input2.trim().parse().expect("Failed to read input ");

    let t = (diagonal1 + diagonal2) / 2;
    println!("The answer is {} cm squared", t);
    return t;
    }

    fn area_of_parallelogram()->i32{
        println!("What is the base in cm ?");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let base:i32 = input1.trim().parse().expect("Failed to read input");

        println!("What is the altitude?");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let altitude:i32  = input2.trim().parse().expect("Failed to read input");

        let t = base * altitude;
        println!("The answer is {} cm squared", t);
        return t;
    }

    fn area_of_cube()->i32{
        println!("What is the length of the cube in cm?");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let length:i32 = input.trim().parse().expect("Failed to read input");

        let t = 6 * (length) ^ 2;
        println!("The answer is {} cm squared", t);
        return t;
    }

    fn volume_of_cylinder()->i32{
        println!("What is the radius in cm?");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let radius:i32 = input1.trim().parse().expect("Failed to read input");

        println!("What is the height in cm?");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let height:i32 = input2.trim().parse().expect("Failed to read input");

        let t = 22 / 7 * (radius) ^ 2 * height;
        println!(" The answer is {}  cm cube",t);
        return t;
    }

