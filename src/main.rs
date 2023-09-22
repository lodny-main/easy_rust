fn main() {
    let mut my_number = 9;
    let num_ref = &mut my_number;
    // let num_ref2 = &my_number;   ->> error

    *num_ref = 10;

    println!("Number is now {my_number}");
}
