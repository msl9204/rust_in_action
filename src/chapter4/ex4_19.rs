use std::rc::Rc;

#[derive(Debug)]
struct GroundStation {}

pub fn example() {
    let base = Rc::new(GroundStation {});

    println!("{:?}", base);
}