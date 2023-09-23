// Option - Maybe there, maybe not
// .is_some()
// .is_none()

// Result - May not work
// .is_ok()
// .is_err()

fn check_error(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
        Ok(())
    } else {
        Err(())
    }
}

fn main() {
    match check_error(5) {
        Ok(_) => println!("It's okay, guys!"),
        Err(_) => println!("It's an error, guys!")
    }
}
