use std::mem::size_of;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

pub fn example() {
    let a: usize = 42;
    
    let b: &[u8; 10] = &B;

    let c: Box<[u8]> = Box::new(C);

    println!("a (an unsigned integer):");
    println!("locations: {:p}", &a);
    println!("size : {:?} bytes", size_of::<usize>());
    println!("value : {:?}", a);
    println!();

    println!("b (a reference to B):");
    println!("locations: {:p}", &b);
    println!("size : {:?} bytes", size_of::<&[u8; 10]>());
    println!("points to : {:?}", b);
    println!();

    println!("c (a \"box\" for C):");
    println!("locations: {:p}", &c);
    println!("size : {:?} bytes", size_of::<Box<[u8]>>());
    println!("points to : {:?}", c);
    println!();

    println!("B (an array of 10 bytes)");
    println!("locations: {:p}", &B);
    println!("size : {:?} bytes", size_of::<Box<[u8; 10]>>());
    println!("points to : {:?}", B);
    println!();

    println!("C (an array of 11 bytes)");
    println!("locations: {:p}", &C);
    println!("size : {:?} bytes", size_of::<Box<[u8; 11]>>());
    println!("points to : {:?}", C);
    println!();
    
}