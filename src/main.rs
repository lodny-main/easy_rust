fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}

fn main() {
    let vec = vec![1, 2, 1, 2, 1, 2];
    // match take_fifth(vec) {
    //     None => println!("There was nothing inside."),
    //     Some(number) => println!("I got a number: {number}")
    // }
    let index = take_fifth(vec);
    if index.is_some() {
        println!("I got a number: {}", index.unwrap());
    }
}
