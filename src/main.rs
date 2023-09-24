// Mutex and RwLock(Read Write)

use std::sync::Mutex;

fn main() {
    let my_mutex = Mutex::new(5);
    println!("{my_mutex:?}");

    let mut mutex_changer = my_mutex.lock().unwrap();
    println!("{my_mutex:?}");

    *mutex_changer = 10;
    drop(mutex_changer);
    println!("{my_mutex:?}");
}
