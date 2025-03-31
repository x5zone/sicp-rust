use std::{cell::RefCell, rc::Rc};

use sicp_rs::prelude::*;
fn math_sqrt(x: List) -> List {
    x.try_as_basis_value::<f64>()
        .expect("sqrt: f64 expected")
        .sqrt()
        .to_listv()
}
fn make_monitored(f: impl Fn(List) -> List) -> impl Fn(List) -> List {
    let count = Rc::new(RefCell::new(0));
    move |x| {
        let mut c = count.borrow_mut();
        if x == "how-many-calls?".to_listv() {
            c.to_owned().to_listv()
        } else {
            *c += 1;
            f(x)
        }
    }
}
fn main() {
    let sqrt = make_monitored(math_sqrt);
    println!("{}", sqrt("how-many-calls?".to_listv()));   // 0.0
    println!("{}", sqrt(100.0.to_listv()));               // 10.0
    println!("{}", sqrt("how-many-calls?".to_listv()));   // 1.0
}
