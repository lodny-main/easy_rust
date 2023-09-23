// take_while()
// cloned()
// by_ref()
// skip_while()
// map_while()
// chunks()
// windows()

fn main() {
    let num_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];

    for chunk in num_vec.chunks(3) {
        println!("{chunk:?}");
    }

    for window in num_vec.windows(3) {
        println!("{window:?}");
    }
}
