use std::mem::size_of_val;

struct SomeStruct;
#[derive(Debug)]
struct Colour(u8, u8, u8);

#[derive(Debug)]
struct Country {
    population: u32,
    capital: String,
}

fn main() {
    let x = SomeStruct;
    println!("The size is {}", size_of_val(&x));

    let c = Colour(20, 50, 100);
    println!("The green value is {}", c.1);
    println!("The green value is {:?}", c);

    let country = Country {
        population: 5000_0000,
        capital: "Seoul".to_string(),
    };
    println!("country: {:#?}", country);
}


