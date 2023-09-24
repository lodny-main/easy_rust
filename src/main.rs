// Mutex : Mutual Exclusion
// Arc : Atomic Reference Counter
// - operating system primitives, thread safe

use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread;

trait CoolTrait {
    fn cool_function(&self);
}

#[derive(Debug)]
struct OurStruct {
    data: RefCell<i32>,
    mutex_data: Mutex<i32>,
    arc_mutex_data: Arc<Mutex<i32>>,
}

impl CoolTrait for OurStruct {
    fn cool_function(&self) {
        *self.data.borrow_mut() += 1;
        *self.mutex_data.lock().unwrap() += 1;
    }
}

fn main() {
    let our_struct = OurStruct {
        data: RefCell::new(0),
        mutex_data: Mutex::new(0),
        arc_mutex_data: Arc::new(Mutex::new(0)),
    };

    let mut join_vec = vec![];
    for _ in 1..=10 {
        // ->> `RefCell<i32>` cannot be shared between threads safely
        // let handle = thread::spawn(|| {     // reference
        //     *our_struct.data.borrow_mut() += 1;
        // });

        // ->> use of moved value: `our_struct.mutex_data`
        // let handle = thread::spawn(move || {    // take a value
        //     *our_struct.arc_mutex_data.lock().unwrap() += 1;
        // });

        let clone = Arc::clone(&our_struct.arc_mutex_data);
        let handle = thread::spawn(move || {    // take a value
            *clone.lock().unwrap() += 1;
            println!("There are {} owners", Arc::strong_count(&clone));
        });

        join_vec.push(handle);
    }

    // for handle in join_vec {
    //     handle.join().unwrap();
    // }
    join_vec
        .into_iter()
        .for_each(|handle| handle.join().unwrap());

    println!("{our_struct:#?}");
}
