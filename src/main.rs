// Mutex and RwLock(Read Write)

use std::sync::RwLock;

fn main() {
    let my_rwlock = RwLock::new(5);
    println!("{my_rwlock:?}");

    let read1 = my_rwlock.read().unwrap();      // lock
    let read2 = my_rwlock.read().unwrap();      // lock
    println!("{read1}, {read2}");

    // like db ?
    let read3 = my_rwlock.read().unwrap();      // lock
    let write1 = my_rwlock.write().unwrap();      // lock
    println!("{read3}, {write1}");
}
