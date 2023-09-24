// Box : owned data on the heap

use std::mem::size_of_val;

struct SomeStruct {
    name: String,
    number: u8,
    data: Box<[u32; 10000]>,
}

fn main() {
    let my_struct = SomeStruct {
        name: "Hi, there".to_string(),
        number: 0,
        data: Box::new([9; 10000]),
    };

    let my_box = Box::new(9);

    println!("{my_box}");
    println!("size of SomeStruct: {}", size_of_val(&my_struct));
}
