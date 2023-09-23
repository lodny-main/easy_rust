use std::collections::BinaryHeap;

fn main() {
    let many_numbers = vec![0, 5, 10, 15, 20, 25, 30];

    let mut my_heap = BinaryHeap::new();

    for number in many_numbers {
        my_heap.push(number);
    }

    // .peek()
    while let Some(number) = my_heap.pop() {
        println!("Popped off {}. Remaining numbers are: {:?}", number, my_heap);
    }
}
