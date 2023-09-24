use std::thread;

fn main() {
    let mut join_vec = vec![];
    for _ in 1..10 {
        let handle = thread::spawn(|| {
            println!("I am printing something");
        });
        join_vec.push(handle);
    }

    // for handle in join_vec {
    //     handle.join().unwrap();
    // }
    join_vec
        .into_iter()
        .for_each(|handle| handle.join().unwrap())
}
