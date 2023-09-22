#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}

#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
}

impl Animal {
    fn new_cat(age: u8) -> Self {       // associated function
        Self {
            age,
            animal_type: AnimalType::Cat
        }
    }
}

fn main() {
    let cat = Animal::new_cat(5);       // associated function
    let Animal { age, .. } = cat;
    println!("age: {}", age);
}

