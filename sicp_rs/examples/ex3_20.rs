use sicp_rs::prelude::*;

fn main() {
    let x =  pair!(1,2);
    let z = pair!(x.clone(),x.clone());
    z.tail().set_head(17.to_listv());

    println!("{}", z);
}