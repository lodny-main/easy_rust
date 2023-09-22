fn give_thing<T>(input: T) -> T {
    input
}

fn main() {
    let x = give_thing(String::from("Take this thing"));
    let y = give_thing(9);

    println!("{x}");
    println!("{y}");
}
