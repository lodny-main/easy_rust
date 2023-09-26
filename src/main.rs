// time
// chrono

use std::time::Instant;

fn main() {
    let now = Instant::now();   // opaque
    println!("{now:?}");

    let time_1 = Instant::now();
    let time_2 = Instant::now();
    println!("time_2 - time_1: {:?}", time_2 - time_1);

    println!("now.elapsed: {:?}", now.elapsed());
}
