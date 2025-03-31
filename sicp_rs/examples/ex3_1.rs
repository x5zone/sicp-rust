use std::{cell::RefCell, rc::Rc};

fn make_accumulator(init: i32) -> impl Fn(i32) -> i32 {
    let sum = Rc::new(RefCell::new(init));
    move |x| {
        let mut s = sum.borrow_mut();
        *s += x;
        *s
    }
}
fn main() {
    let a = make_accumulator(5);
    println!("{}", a(10));
    println!("{}", a(10));
}
