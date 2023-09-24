// impl trait
// return closure

fn return_a_closure() -> Box<dyn Fn(i32)> {
    Box::new(|x| println!("{x}"))
}

fn return_a_closure_new() -> impl Fn(i32) {
    |x| println!("{x}")
}

fn main() {
    let my_number = 9;

    let closure = return_a_closure();
    closure(my_number);

    let closure_new = return_a_closure_new();
    closure_new(my_number);
}
