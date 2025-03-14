use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
}

pub enum List {
    Cons(Rc<RefCell<Node>>, Weak<RefCell<Node>>),
    Nil,
}

// 定义函数创建 Cons
fn create_cons() -> List {
    let node1 = Rc::new(RefCell::new(Node { value: 1 }));
    let node2 = Rc::new(RefCell::new(Node { value: 2 }));

    // 创建 Cons
    let cons = List::Cons(node1.clone(), Rc::downgrade(&node2));

    // 打印引用计数
    println!("Inside create_cons:");
    println!("node1 strong_count: {} (held by node1 & cons)", Rc::strong_count(&node1));
    println!("node2 strong_count: {} (held by node2)", Rc::strong_count(&node2));
    println!("node2 weak_count: {} (held by cons)", Rc::weak_count(&node2));

    cons // 返回 Cons
}

fn main() {
    let cons = create_cons(); // 调用函数创建 Cons

    // 打印引用计数
    println!("\nInside main:");
    match &cons {
        List::Cons(node1, node2) => {
            println!(
                "node1 strong_count: {} (held by node1 & cons)",
                Rc::strong_count(node1)
            );
            println!(
                "node2 strong_count: {} (held by node2)",
                Rc::strong_count(&node2.upgrade().unwrap())
            );
            println!(
                "node2 weak_count: {} (held by cons)",
                Rc::weak_count(&node2.upgrade().unwrap())
            );
        }
        List::Nil => println!("Cons is Nil"),
    }
}
