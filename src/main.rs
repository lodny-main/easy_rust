use std::num::ParseIntError;

fn parse_number(number: &str) -> Result<i32, ParseIntError> {
    number.parse()
}

fn main() {
    let mut result = vec![];
    result.push(parse_number("8"));
    result.push(parse_number("dasfas"));
    result.push(parse_number("108"));

    for number in result {
        println!("{:?}", number);
    }
}
