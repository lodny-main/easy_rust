// .any()
// .all()
// .rev()
// .find()  : I'll try to get it for you
// .position()  : I'll try to find the position for you
// .cycle()
// .take()

fn main() {
    let even_odd = vec!["even", "odd"];

    let even_odd_vec = (0..6)
        .zip(even_odd.into_iter().cycle())
        .collect::<Vec<(i32, &str)>>();

    println!("{even_odd_vec:?}");
}
