// closure : anonymous functions that capture their environment
// zero cost abstractions

fn main() {
    let my_number = 10;

    let my_closure = |x: i32| println!("{}", x + my_number);

    my_closure(10);
}
