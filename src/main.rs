fn main() {
    let string_1 = String::from("Hello there");  // From trait
    let string_2 = "Hello there".to_string();       // Display trait

    // type annotations needed
    // let string_3 = "Hello there".into();
    // From
    let string_3: String = "Hello there".into();

    // ToOwned trait : str -> String
    let string_4 = "Hello there".to_owned();

    // Clone : &T -> T, &str -> str
}
