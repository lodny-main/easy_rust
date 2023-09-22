fn main() {
    let mut input = 5;
    match_number(input);

    input = 15;
    match_number(input);

    input = 25;
    match_number(input);
}

fn match_number(input: i32) {
    match input {
        0..=10 => { println!("true {}", input); }
        number @ 11..=20 => println!("true {}, {}", input, number),
        _ => println!("false")
    }
}


