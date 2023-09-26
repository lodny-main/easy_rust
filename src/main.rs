// channel
// mpsc : multiple producer - single consumer

use std::sync::mpsc::channel;

fn main() {
    let (tx, rx) = channel();

    tx.send(9).unwrap();
    let received = rx.recv().unwrap();

    println!("received: {received}");
}
