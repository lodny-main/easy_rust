fn main() {
    let big_string = "Hello there, I am a &str";

    big_string
        .char_indices()     // chars().enumerate()
        .for_each(|(index, charrr)| {
            println!("At index {index} is the char {charrr}");
        });
}
