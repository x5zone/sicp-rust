use std::{cell::RefCell, rc::Rc};

use sicp_rs::prelude::*;

fn make_queue() -> impl Fn(&str) -> ClosureWrapper {
    let front_ptr = Rc::new(RefCell::new(List::Nil));
    let rear_ptr = Rc::new(RefCell::new(List::Nil));

    let set_front_ptr = {
        let front_ptr = front_ptr.clone();
        move |item: List| *front_ptr.clone().borrow_mut() = item
    };
    let set_rear_ptr = {
        let rear_ptr = rear_ptr.clone();
        move |item: List| *rear_ptr.clone().borrow_mut() = item
    };
    let is_empty_queue = {
        let front_ptr = front_ptr.clone();
        move || front_ptr.borrow().is_empty()
    };
    let insert_queue = {
        let is_empty_queue = is_empty_queue.clone();
        let rear_ptr = rear_ptr.clone();
        let set_rear_ptr = set_rear_ptr.clone();
        let set_front_ptr = set_front_ptr.clone();
        move |item: List| {
            let new_pair = pair![item, List::Nil];
            if is_empty_queue() {
                set_front_ptr(new_pair.clone());
                set_rear_ptr(new_pair.clone());
            } else {
                rear_ptr.borrow().set_tail(new_pair.clone());
                set_rear_ptr(new_pair.clone());
            }
        }
    };
    let delete_queue = {
        let is_empty_queue = is_empty_queue.clone();
        let front_ptr = front_ptr.clone();
        let set_front_ptr = set_front_ptr.clone();
        move || {
            if is_empty_queue() {
                panic!("delete_queue called with an empty queue");
            } else {
                let new_front_ptr = front_ptr.borrow().tail();
                set_front_ptr(new_front_ptr.clone());
            }
        }
    };

    let dispatch = move |msg: &str| match msg {
        "insert_queue" => {
            let insert_queue = insert_queue.clone();
            ClosureWrapper::new(move |item: &List| {
                insert_queue(item.clone());
                Some("ok".to_listv())
            })
        }
        "delete_queue" => {
            let delete_queue = delete_queue.clone();
            ClosureWrapper::new(move |_| {
                delete_queue();
                Some("ok".to_listv())
            })
        }
        "print_queue" => {
            let front_ptr = front_ptr.clone();
            ClosureWrapper::new(move |_| {
                println!("{}", front_ptr.borrow());
                Some("ok".to_listv())
            })
        }
        _ => panic!("unknown message"),
    };
    dispatch
}
fn main() {
    let q1 = make_queue();
    q1("insert_queue").call(&"a".to_listv());
    q1("print_queue").call(&List::Nil);
    q1("insert_queue").call(&"b".to_listv());
    q1("print_queue").call(&List::Nil);
    q1("delete_queue").call(&List::Nil);
    q1("print_queue").call(&List::Nil);
    q1("delete_queue").call(&List::Nil);
    q1("print_queue").call(&List::Nil);
}
