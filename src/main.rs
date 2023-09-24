// 3 generics

use std::fmt::Display;

// concrete : generate using type functions at compile time."
// compile : slow
// run : fast
fn print_1<T: Display>(input: T) {
    println!("Hi, I'm a {input}");
}

// concrete
fn print_2(input: impl Display) {
    println!("Hi, I'm a {input}");
}

// dynamic : a little slow
fn print_3(input: &dyn Display){
    println!("Hi, I'm a {input}");
}

// dynamic
fn print_4(input: Box<dyn Display>){
    println!("Hi, I'm a {input}");
}


fn main() {
    print_1(9);
    print_2(5);
    print_3(&2);
    print_4(Box::new(7));
}
