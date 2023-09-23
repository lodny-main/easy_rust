// chaining methods and functional style

fn main() {
    let my_vec = (1..=10).collect::<Vec<_>>();
    println!("my_vec: {:?}", my_vec);

    let new_vec = my_vec
        .into_iter()
        .skip(3)
        .take(5)
        .collect::<Vec<i32>>();
    println!("new_vec: {:?}", new_vec);
    // println!("my_vec: {:?}", my_vec);        // after move
}
