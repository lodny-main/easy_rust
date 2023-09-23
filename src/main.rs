use std::collections::HashMap;

fn main() {
    let book_vec = vec![
        "book 1",
        "book 2",
        "book 3",
        "book 4",
        "book 3",
        "book 3",
        "book 1",
    ];

    let mut book_hashmap = HashMap::new();

    for book in book_vec {
        // let number_of_books = book_hashmap.entry(book).or_insert(0);
        // *number_of_books += 1;

        book_hashmap
            .entry(book)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    for (book, number) in book_hashmap {
        println!("{}: {} copies", book, number);
    }
}
