fn print_number(number: i32) {
    println!("{number}");
}

fn print_string(input: String) {
    println!("{input}");
}

fn main() {
    // Copy type
    let my_number = 8;
    print_number(my_number);
    print_number(my_number);

    // Non Copy type : clone
    let my_country = "Austria".to_string();
    print_string(my_country.clone());
    print_string(my_country);
}


