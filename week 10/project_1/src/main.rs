struct Total {
     amount_of_HP:u32, amount_of_IBM:u32, amounrt_of_toshiba:u32, amount_of_Dell:u32,  quantity_of_laptop:u32
}

impl Total {
    fn total(&self)->u32 {
        self.amount_of_HP * self.quantity_of_laptop + self.amount_of_IBM * self.quantity_of_laptop + self.amounrt_of_toshiba * self.quantity_of_laptop + self.amount_of_Dell * self.quantity_of_laptop
    }
}

fn main() {
    let small = Total {
    amount_of_HP:650000,
    amount_of_IBM:755000,
    amounrt_of_toshiba:550000,
    amount_of_Dell:850000,
    quantity_of_laptop:3
    };
    println!("The total amount of all the laptops is {}",small.total());
}