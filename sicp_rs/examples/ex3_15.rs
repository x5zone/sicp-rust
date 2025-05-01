use sicp_rs::prelude::*;

fn set_to_wow(x: List) -> List {
    x.head().set_head("wow".to_listv());
    x
}

fn main() {
    let x = list!["a", "b"];
    let z1 = pair![x.clone(), x.clone()];
    let z2 = pair![list!["a", "b"], list!["a", "b"]];
    println!("z1: {}\nz2: {}", z1, z2);
    println!("set_to_wow(z1): {}", set_to_wow(z1));
    println!("set_to_wow(z2): {}", set_to_wow(z2))
}
// z1: ((a, (b, Nil)), (a, (b, Nil)))
// z2: ((a, (b, Nil)), (a, (b, Nil)))
// set_to_wow(z1): ((wow, (b, Nil)), (wow, (b, Nil)))
// set_to_wow(z2): ((wow, (b, Nil)), (a, (b, Nil)))
