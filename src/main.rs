// blanket trait implementations
// implement trait for all types that you want to have it

use std::fmt::{Debug, Display};

trait Prints {
    fn debug_print(&self) where Self: Debug {
        println!("Debug ->> I am: {:?}", self);
    }

    fn display_print(&self) where Self: Display {
        println!("Display ->> I am: {}", self);
    }
}

#[derive(Debug)]
struct Person;
#[derive(Debug)]
struct Building;

impl<T> Prints for T {
}


fn main() {
    let person = Person;
    let building = Building;
    let my_string = String::from("Hello");

    person.debug_print();
    building.debug_print();
    my_string.debug_print();
    my_string.display_print();
}
