use std::rc::{Rc, Weak};

fn main() {
    let rc_data = Rc::new(43);
    println!("{}", Rc::strong_count(&rc_data));

    let weak_data = Rc::downgrade(&rc_data);
    println!("{}", Rc::strong_count(&rc_data));
    println!("{:?}", Weak::upgrade(&weak_data));

    drop(rc_data);
    println!("{:?}", Weak::upgrade(&weak_data));
}
