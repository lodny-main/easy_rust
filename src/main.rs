// String
// &str
// string literal
// borrowed str

fn main() {
    let my_name = "Juice";
    let my_string = "Coco".to_string();      // &'static - for the life of the program
    let my_string_ref = &my_string;         // &str - reference to something else
}
