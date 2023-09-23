enum LibraryType {
    City,
    Country,
}

struct Library {
    library_type: LibraryType,
    books: Vec<String>,
}

impl Library {
    fn new() -> Self {
        Self {
            library_type: LibraryType::City,
            books: Vec::new(),
        }
    }

    fn add_book(&mut self, book: &str) {
        self.books.push(book.to_string());
    }
}

impl Iterator for Library {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.books.pop()
    }
}

fn main() {
    let mut my_library = Library::new();
    my_library.add_book("The Doom");
    my_library.add_book("Demian");
    my_library.add_book("구운몽");
    my_library.add_book("데이터베이스");

    println!("{:?}", my_library.books);

    for book in my_library {
        println!("{}", book);
    }
}
