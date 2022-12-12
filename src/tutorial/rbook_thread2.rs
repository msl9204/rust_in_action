use std::thread;
use std::time::Duration;

pub fn example() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector : {:?}", v);
    });

    // drop(v);

    handle.join().unwrap();
}