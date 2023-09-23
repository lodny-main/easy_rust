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
    if check_error(5).is_ok() {
        println!("It's okay, guys!");
    } else {
        println!("It's an error, guys!");
    }
}
