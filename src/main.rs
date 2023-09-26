// time
// chrono

use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    let now = Instant::now();   // opaque
    println!("{now:?}");

    sleep(Duration::from_secs(3));
    println!("now.elapsed: {:?}", now.elapsed());
}
