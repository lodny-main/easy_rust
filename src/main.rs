fn check_if_five(number: i32) -> Result<i32, String> {
    match number {
        5 => Ok(number),
        _ => Err("Sorry, the number wasn't five.".to_string())
    }
}

fn main() {
    let mut result = Vec::new();

    for number in 2..=7 {
        result.push(check_if_five(number));
    }

    println!("{:#?}", result);
}
