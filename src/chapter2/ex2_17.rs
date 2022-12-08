use std::time::Duration;

fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

pub fn example() {
    let floats = add(1.2, 3.4);
    let ints = add(10, 20);
    let durations = add(
        Duration::new(5, 0),
        Duration::new(10, 0)
    );

    println!("{}", floats);
    println!("{}", ints);
    println!("{:?}", durations);
}