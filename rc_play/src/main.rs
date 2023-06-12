use std::rc::Rc;

enum List {
    Cons(i32, Box<Rc<List>>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Box::new(Rc::new(Cons(10, Box::new(Rc::new(Nil)))))));
    println!("{}", Rc::strong_count(&a));
    let c = Cons(2, Box::new(Rc::clone(&a)));
    println!("{}", Rc::strong_count(&a));
    {
        let b = Cons(1, Box::new(Rc::clone(&a)));
        println!("{}", Rc::strong_count(&a));
    }
    println!("{}", Rc::strong_count(&a));

}