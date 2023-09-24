// recursive types

#[derive(Debug)]
struct List {
    // content: List,      // --> error: recursive type `List` has infinite size
    content: Box<List>,
}

#[derive(Debug)]
enum ListEnum {
    Content(i32, Box<ListEnum>),
    NoContent,
}

fn main() {
    let list_enum = ListEnum::Content(213, Box::new(ListEnum::NoContent));

    println!("{list_enum:?}");
}
