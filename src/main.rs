use std::num::ParseIntError;

fn parse_str(input: &str) -> Result<(), ParseIntError> {
    let parsed_number = input.parse::<i32>()?;  // chaining 가능
    println!("It worked! {:?}", parsed_number);

    Ok(())
}

fn main() {
    for item in vec!["Seven", "8", "9.0", "nice", "6060"] {
        parse_str(item);
    }
}
