pub fn example() {

}

pub fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {
    i + j
}