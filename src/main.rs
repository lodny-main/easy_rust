// .skip()
// .take()
// .fold() : javascript : reduce()
// .sum()

fn main() {
    let ten_chars = ('a'..)
        .skip(10)
        .take(10)
        .collect::<Vec<_>>();
    println!("{ten_chars:?}");

    let a_string = "I don't have any dashes in me.";
    let dashed = a_string
        .chars()
        .fold("-".to_string(), |mut string_to_far, next_char| {
            string_to_far.push(next_char);
            string_to_far.push('-');
            string_to_far
        });
    println!("{dashed:?}");
}
