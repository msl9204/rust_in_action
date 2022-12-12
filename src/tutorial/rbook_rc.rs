enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;
use List::*;

pub fn example() {
    let a = Rc::new(
        Cons(5, Rc::new(
            Cons(10, Rc::new(Nil))
        ))
    );

    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));


    let d = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating d = {}", Rc::strong_count(&d));
    let e = Cons(3, Rc::clone(&d));
    println!("count after creating e = {}", Rc::strong_count(&d));

    {
        let f = Cons(4, Rc::clone(&d));
        println!("count after creating f = {}", Rc::strong_count(&d));
    }
    
    println!("count after c goes out of scope f = {}", Rc::strong_count(&d));
}