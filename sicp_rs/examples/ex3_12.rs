use sicp_rs::prelude::*;

fn main() {
    let x = list![1, 2];
    let y = list![3, 4];
    let z = x.append(&y);
    println!("{}", z);
    println!("{}", x.tail());

    x.last_pair().set_tail(y.clone());

    println!("{}", x.tail());
}
// (1, (2, (3, (4, Nil))))
// (2, Nil)
// (2, (3, (4, Nil)))