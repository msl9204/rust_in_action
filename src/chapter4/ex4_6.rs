fn use_value(_val: Demo) {
    
}

struct Demo {
    a: i32
}

pub fn example() {
    let demo = Demo { a: 32 };
    use_value(demo);

    // println!("{}", demo.a);
}