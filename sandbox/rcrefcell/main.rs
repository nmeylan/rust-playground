use std::cell::RefCell;
use std::rc::Rc;

struct A {
    x: u16,
    y: u16,
}

impl A {

}

fn main() {
    let a = Rc::new(RefCell::new(A{ x: 0, y: 0 }));
    let a_ref = a.borrow();
    println!("{:?}", a_ref.x);
}