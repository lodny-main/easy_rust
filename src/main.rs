// RefCell
// - runtime checked borrowing rules
// - borrow_mut()
// - try_borrow_mut()

use std::cell::RefCell;

#[derive(Debug)]
struct User {
    id: u32,
    year_registered: u32,
    username: String,
    active: RefCell<bool>
}

fn main() {
    let user_1 = User {
        id: 1,
        year_registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true),
    };
    println!("{user_1:?}");

    let mut ref_mut_1 = user_1.active.borrow_mut();     // ->> status : borrowed
    // let ref_mut_2 = user_1.active.borrow_mut();      // panic
    println!("{user_1:?}, {ref_mut_1} <-- after borrow_mut()");

    *ref_mut_1 = false;
    println!("{user_1:?}, {ref_mut_1} <-- after assign false");

    drop(ref_mut_1);
    println!("{user_1:?} <-- after drop()");
}
