use std::fmt::Display;

fn generic_function<T: Display>(input: T) {
    println!("{input}");
}

fn impl_function(input: impl Display) {
    println!("{input}");
}

fn main() {
    generic_function(8);
    generic_function::<u8>(8);

    impl_function(9);
    // impl_function::<u8>(9);
    // function takes 0 generic arguments but 1 generic argument was supplied
}
