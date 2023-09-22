enum Number {
    U32(u32),
    I32(i32)
}


fn main() {
    let vec = vec![get_number(-800), get_number(8)];
    for item in vec {
        match item {
            Number::U32(number) => println!("u32: {}", number),
            Number::I32(number) => println!("i32: {}", number)
        }
    }
}

fn get_number(input: i32) -> Number {
    match input.is_positive() {
        true => Number::U32(input as u32),
        false => Number::I32(input)
    }
}
