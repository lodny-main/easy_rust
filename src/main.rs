// channel
// mpsc : multiple producer - single consumer

use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let (tx, rx) = channel();

    let s1 = tx.clone();
    let s2 = tx.clone();
    thread::spawn(move || {
        s1.send(9).unwrap();
    });

    thread::spawn(move || {
        s2.send(3).unwrap();
    });

    println!("received: {}", rx.recv().unwrap());
    println!("received: {}", rx.recv().unwrap());
}
