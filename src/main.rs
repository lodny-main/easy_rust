fn main() {
    'first: loop {
        'second: loop {
            break 'first;
        }
    }

    for number in 0..3 {    // Range
        println!("number: {}", number);
    }

    let mut counter = 5;
    let my_number = loop {
        counter += 1;
        if counter % 53 == 3 {
            break counter;
        }
    };

    println!("my_number is now: {}", my_number);
}
