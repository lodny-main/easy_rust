// interior mutability
// changing on the inside

// &    : immutable reference / shared reference
// &mut : mutable reference / unique reference

// Cell : not mutable but changeable
// impl<T: ?Sized> !Sync for Cell<T> {} ->> !Sync
// RefCell
// Mutex
// RwLock

use std::cell::Cell;


#[derive(Debug)]
struct TestCell {
    num: Cell<i32>,
}

fn main() {
    let cell = TestCell {
        num: Cell::new(10),
    };
    println!("{cell:?}");

    cell.num.set(20);
    println!("{cell:?}");
}
