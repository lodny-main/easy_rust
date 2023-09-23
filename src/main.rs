use std::collections::HashMap;

fn main() {
    let some_numbers = vec![0, 1, 2, 3, 4, 5];
    let some_words = vec!["zero", "one", "two", "three", "four", "five"];

    let map = some_numbers
        .iter()
        .zip(some_words)
        .collect::<HashMap<_, _>>();

    println!("HashMap: {:?}", map);
}
