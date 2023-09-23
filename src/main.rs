// Option - Maybe there, maybe not
// .is_some()
// .is_none()
// None.unwrap() ->> panic

// Result - May not work
// .is_ok()
// .is_err()
// Err.unwrap() ->> panic

fn check_error(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
        Ok(())
    } else {
        Err(())
    }
}

fn main() {
    check_error(5).unwrap();
}

