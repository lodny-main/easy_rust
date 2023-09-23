fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}

fn main() {
    let vec = vec![1, 2];
    let index = take_fifth(vec);

    index.expect("Needed at least five items");
}
