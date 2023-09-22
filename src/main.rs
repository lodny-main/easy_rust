use std::fmt::{Display, Formatter};

struct Book;

impl Display for Book {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        println!("()");
        Ok(())
    }
}

fn give_thing<T: Display>(input: T) -> T {
    println!("{}", input);
    input
}

fn main() {
    let x = give_thing(String::from("Take this thing"));
    let y = give_thing(9);
    let z = give_thing(Book);

    println!("{x}");
    println!("{y}");
    println!("{z}");
}
