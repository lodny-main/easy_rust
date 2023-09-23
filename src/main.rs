struct Book<'a> {
    name: &'a str,
}

fn returns_reference() -> &'static str {
    "Juice"
}

fn main() {
    let reference = returns_reference();
    println!("{reference}");

    let my_book = Book {
        name: "my book"
    };
}
