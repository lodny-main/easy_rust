fn main() {
    let my_vec = vec![2, 3, 4];
    let one = my_vec.get(0);
    let two = my_vec.get(10);

    println!("{:?}, {:?}", one, two);
}
