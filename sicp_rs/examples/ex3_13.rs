use sicp_rs::prelude::*;

fn make_cycle(x: List) -> List {
    x.last_pair().set_tail(x.clone());
    x
}
fn main() {
    let x = list![1, 2, 3];
    let y = make_cycle(x.clone());

    let mut z = y.clone();
    for _ in 0..10 {
        print!("{} ", z.head());
        z = z.tail();
    }
    println!();
    // y.last_pair(); // will panic
    test();
}
// 1 2 3 1 2 3 1 2 3 1
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub enum NewList {
    Cons(Rc<RefCell<i32>>, Weak<RefCell<i32>>),
    Nil,
}

fn create_cons() -> NewList {
    let node1 = Rc::new(RefCell::new(1));
    let node2 = Rc::new(RefCell::new(2));

    let cons = NewList::Cons(node1.clone(), Rc::downgrade(&node2));

    println!("Inside create_cons:");
    println!(
        "node1 strong_count: {} (held by node1 & cons)",
        Rc::strong_count(&node1)
    );
    println!(
        "node2 strong_count: {} (held by node2)",
        Rc::strong_count(&node2)
    );
    println!(
        "node2 weak_count: {} (held by cons)",
        Rc::weak_count(&node2)
    );

    cons
}

fn test() {
    let cons = create_cons(); 

    println!("\nInside test:");
    match &cons {
        NewList::Cons(node1, node2) => {
            println!(
                "node1 strong_count: {} (held by cons)",
                Rc::strong_count(&node1)
            );
            println!(
                "node2 value: {:?} upgrade: {:?} weak_count: {}",
                node2,
                node2.upgrade(),
                node2.weak_count()
            );
        }
        NewList::Nil => println!("Cons is Nil"),
    }
}
// Inside create_cons:
// node1 strong_count: 2 (held by node1 & cons)
// node2 strong_count: 1 (held by node2)
// node2 weak_count: 1 (held by cons)

// Inside test:
// node1 strong_count: 1 (held by cons)
// node2 value: (Weak) upgrade: None weak_count: 0
