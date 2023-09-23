// and_then()   :: flat_map()

use std::arch::x86_64::__cpuid;

fn main() {
    let some_output = Some(vec![8, 9, 10]);
    // let some_value: Option<Vec<i32>> = None;

    let first = some_output
        .clone()
        .map(|some_vec| {
            some_vec.iter().map(|num| num + 1).collect::<Vec<_>>()
        });
    println!("{first:?}");

    let second = some_output
        .clone()
        .map(|some_vec| match some_vec.len() {
            0 => None,
            1 => Some(vec![some_vec[0]]),
            _ => Some(some_vec)
        });
    println!("{second:?}");

    let third = some_output
        .and_then(|some_vec| match some_vec.len() {
            0 => None,
            1 => Some(vec![some_vec[0]]),
            _ => Some(some_vec)
        });
    println!("{third:?}");

}
