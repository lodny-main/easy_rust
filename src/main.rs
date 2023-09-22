use std::panic::panic_any;

#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}

// impl block for enum
impl AnimalType {
    fn check_type(&self) {
        use AnimalType::*;

        match self {
            Cat => println!("Animal type is cat"),
            Dog => println!("Animal type is dog")
        }
    }
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

    fn new_dog(age: u8) -> Self {
        Self {
            age,
            animal_type: AnimalType::Dog
        }
    }

    pub(crate) fn print(&self) {        // method
        println!("animal: {:?}", self);
    }
}

fn main() {
    let cat = Animal::new_cat(5);       // associated function
    cat.print();

    let dog = Animal::new_dog(8);       // associated function
    dog.print();                                    // syntactic sugar : dot operator
    Animal::print(&dog);

    dog.animal_type.check_type();
}

