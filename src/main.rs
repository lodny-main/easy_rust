fn main() {
    let vec1 = vec![2, 4, 6];

    vec1
        .iter()
        .enumerate()    // (0, item1), (1, item2) tuple
        .for_each(|(index, number)| {
            println!("the number at index {index} is {number}");
        });
}
