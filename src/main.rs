// Iterator : a collection of things that you can call .next() on
// .iter() : iterator of references &T
// .iter_mut() : iterator of mutable references &mut T
// .into_iter() : consuming iterator

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec1_a = vec1
        .iter()
        .map(|x| x + 1)
        .collect::<Vec<i32>>();
    let vec1_b: Vec<i32> = vec1
        .into_iter()
        .map(|x| x * 10)
        .collect();

    let mut vec2 = vec![10, 20, 30];
    vec2
        .iter_mut()
        .for_each(|num| *num += 100);

    println!("vec1_a: {:?}", vec1_a);
    println!("vec1_b: {:?}", vec1_b);
    println!("vec2: {:?}", vec2);
    // println!("vec1: {:?}", vec1);    // error happened : after moved
}
