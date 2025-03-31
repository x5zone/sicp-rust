use sicp_rs::prelude::*;
fn math_sqrt(x: List) -> List {
    x.try_as_basis_value::<f64>()
        .expect("sqrt: f64 expected")
        .sqrt()
        .to_listv()
}
fn make_monitored(f: impl Fn(List) -> List) -> impl FnMut(List) -> List {
    let mut count = 0;
    move |x| {
        if x == "how-many-calls?".to_listv() {
            count.to_listv()
        } else {
            count += 1;
            f(x)
        }
    }
}
fn main() {
    let mut sqrt = make_monitored(math_sqrt);
    println!("{}", sqrt("how-many-calls?".to_listv())); // 0
    println!("{}", sqrt(100.0.to_listv())); // 10.0
    println!("{}", sqrt("how-many-calls?".to_listv())); // 1
}
