// Deref

use std::ops::{Deref, DerefMut};

struct HoldsANumber(u8);

// smart pointer
impl Deref for HoldsANumber {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HoldsANumber {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// DerefMut

fn main() {
    let value = 7;
    let reference = &7;

    println!("{}", value == *reference);

    let mut my_number = HoldsANumber(20);
    // type `HoldsANumber` cannot be dereferenced
    println!("{}", *my_number + 10);
    // can use dot method
    println!("{}", my_number.checked_add(10).unwrap() + 10);

    *my_number = 50;
    println!("{}", *my_number);
}
