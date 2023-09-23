fn main() {
    let vec1 = vec![2, 4, 6];

    let map = vec1
        .iter()
        .map(|num| num * 2)
        .map(|num| num * 3)
        .map(|num| num * 4);
    println!("{:?}", map);

    let vec2 = map
        .collect::<Vec<_>>();

    println!("{:?}", vec2);
}
