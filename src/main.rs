trait Booky {}

struct Book;
impl Booky for Book {}

struct BigBook;
impl Booky for BigBook {}

fn main() {
    // let vec_of_booky_things: Vec<Booky> = vec![Book, BigBook];
    // --> the size for values of type `dyn Booky` cannot be known at compilation time

    // dyn : dynamically
    let vec_of_booky_things: Vec<Box<dyn Booky>> = vec![Box::new(Book), Box::new(BigBook)];
}
