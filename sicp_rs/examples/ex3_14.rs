use sicp_rs::prelude::*;
fn mystery(x: List) -> List {
    fn loopf(x: List, y: List) -> List {
        if x.is_empty() {
            y
        } else {
            let temp = x.tail();
            x.set_tail(y);
            loopf(temp, x)
        }
    }
    loopf(x, List::Nil)
}
fn main() {
    let v = list![1, 2, 3, 4];
    let w = mystery(v.clone());
    println!("v: {}\nw: {}", v, w);
    // v: (1, Nil)
    // w: (4, (3, (2, (1, Nil))))
}
