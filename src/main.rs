fn gives_five() -> u8 {
    5
}

fn gives_six() -> u8 {
    6
}

fn add_to_function_output(my_func: fn() -> u8, some_number: u8) {
    let my_number = my_func();
    let next_number = my_number + some_number;

    println!("We got {next_number}");
}

fn main() {
    add_to_function_output(gives_five, 100);
    add_to_function_output(gives_six, 100);
}


