// Cell
// - not mutable but changeable
// - not thread safe
// - small copy types
// get()
// set()

use std::cell::Cell;

fn main() {
    let my_cell = Cell::new(String::from("I am a String"));

    my_cell.set(String::from("I am a String????"));
    let my_string = my_cell.get();
}
