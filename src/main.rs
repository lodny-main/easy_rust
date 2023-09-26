// channel
// mpsc : multiple producer - single consumer

use std::sync::mpsc::channel;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn sleepy(time: u64) {
    sleep(Duration::from_millis(time));
}

fn main() {
    let (tx, rx) = channel();

    let s1 = tx.clone();
    let s2 = tx.clone();
    thread::spawn(move || {
        sleepy(1000);
        s1.send(9).unwrap();
    });

    thread::spawn(move || {
        sleepy(1000);
        s2.send(3).unwrap();
    });

    // println!("received: {}", rx.recv().unwrap());   // blocking
    // println!("received: {}", rx.recv().unwrap());

    // println!("received: {:?}", rx.try_recv());
    // println!("received: {:?}", rx.try_recv());
    println!("received: {:?}", rx.recv_timeout(Duration::from_millis(500)));
}
