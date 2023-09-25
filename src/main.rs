// java: typeof
// downcasting -> dynamically making concrete
// &dyn Any

use std::any::{Any, type_name};

struct MyType;

fn get_type_name<T: Any>(_: T) {
    let my_type = type_name::<T>();
    println!("{my_type}");
}

fn main() {
    get_type_name(8);
    get_type_name(vec![8]);
    get_type_name(MyType);
}
