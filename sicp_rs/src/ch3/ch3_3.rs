use crate::list_impl::panic_with_location;
use crate::prelude::*;
use std::rc::Rc;

/// 3.3.2 队列的表示
pub fn make_queue() -> List {
    pair!(List::Nil, List::Nil)
}
pub fn is_empty_queue(queue: &List) -> bool {
    front_ptr(queue).is_empty()
}
pub fn insert_queue(queue: &List, item: List) -> List {
    let new_pair = pair!(item, List::Nil);
    if is_empty_queue(queue) {
        set_front_ptr(queue, new_pair.clone());
        set_rear_ptr(queue, new_pair.clone());
        queue.clone()
    } else {
        rear_ptr(queue).set_tail(new_pair.clone());
        set_rear_ptr(queue, new_pair.clone());
        queue.clone()
    }
}
pub fn delete_queue(queue: &List) -> List {
    if is_empty_queue(queue) {
        panic_with_location("delete_queue called with an empty queue", queue)
    } else {
        set_front_ptr(queue, front_ptr(queue).tail());
        queue.clone()
    }
}
fn front_ptr(queue: &List) -> List {
    queue.head()
}
fn rear_ptr(queue: &List) -> List {
    queue.tail()
}
fn set_front_ptr(queue: &List, item: List) -> List {
    queue.set_tail(item);
    queue.clone()
}
fn set_rear_ptr(queue: &List, item: List) -> List {
    queue.set_head(item);
    queue.clone()
}
pub fn front_queue(queue: &List) -> List {
    if is_empty_queue(queue) {
        panic_with_location("front_queue called with an empty queue", queue)
    } else {
        front_ptr(queue).head()
    }
}

/// 3.3.3 表格的表示

fn assoc(key: &List, records: &List) -> Option<List> {
    assert!(
        key.is_value() && key.is_string_value(),
        "assoc key must be string"
    );

    if records.is_empty() {
        None
    } else if *key == records.head().head() {
        // List::V(_) == List::V(_)
        Some(records.head().clone())
    } else {
        assoc(key, &records.tail())
    }
}
pub fn lookup(key: &List, table: &List) -> Option<List> {
    let record = assoc(key, &table.tail());
    if let Some(record) = record {
        Some(record.tail())
    } else {
        None
    }
}
pub fn insert(key: &List, value: List, table: &List) -> Option<List> {
    let record = assoc(&key, &table.tail());
    if let Some(record) = record {
        record.set_tail(value);
    } else {
        table.set_tail(pair!(pair!(key.clone(), value), table.tail()))
    }
    Some("ok".to_string().to_listv())
}
pub fn make_table() -> List {
    list!["*table*"]
}
#[cfg(test)]
mod test_table_1d {
    use super::*;

    #[test]
    fn test_insert() {
        let t = make_table();
        println!("{}", t);
    }
}

pub fn lookup_2d(key1: &List, key2: &List, local_table: &List) -> Option<List> {
    let subtable = assoc(&key1, &local_table.tail());

    if let Some(subtable) = subtable {
        let record = assoc(&key2, &subtable.tail());
        if let Some(record) = record {
            Some(record.tail())
        } else {
            None
        }
    } else {
        None
    }
}
pub fn insert_2d(key1: &List, key2: &List, value: List, local_table: List) -> Option<List> {
    let subtable = assoc(&key1, &local_table.tail());
    if let Some(subtable) = subtable {
        let record = assoc(&key2, &subtable.tail());
        if let Some(record) = record {
            record.set_tail(value);
        } else {
            subtable.set_tail(pair!(pair!(key2.clone(), value), subtable.tail()));
        }
    } else {
        local_table.set_tail(pair!(
            list!(key1.clone(), pair!(key2.clone(), value)),
            local_table.tail()
        ));
    }
    Some("ok".to_string().to_listv())
}

pub fn make_table_2d() -> Rc<dyn Fn(&str) -> ClosureWrapper> {
    let local_table = Rc::new(list!["*table*"]);
    let local2 = local_table.clone();
    // 必须将闭包显式写在此处,以方便编译器推断生命周期;若这部分代码直接写在下面的闭包中,则编译器无法推断生命周期,编译失败.
    let lookup = move |args: &List| {
        let a1 = args.head();
        let a2 = args.tail().head();
        let lt = local_table.clone();

        lookup_2d(&a1, &a2, &lt)
    };
    let insert = move |args: &List| {
        let a1 = args.head();
        let a2 = args.tail().head();
        let a3 = args.tail().tail().head();
        let lt = local2.clone();

        insert_2d(&a1, &a2, a3, (*lt).clone())
    };
    Rc::new(move |m: &str| {
        if m == "lookup" {
            ClosureWrapper::new(lookup.clone())
        } else if m == "insert" {
            ClosureWrapper::new(insert.clone())
        } else {
            panic!("unknown message")
        }
    })
}

#[cfg(test)]
mod test_table_2d {
    use super::*;
    #[test]
    fn test_insert() {
        let optable = make_table_2d();
        let get = |args: List| optable("lookup").call(&args);
        let put = |args: List| optable("insert").call(&args);
        put(list!["deriv", "+", 3]);
        put(list!["deriv", "+", 3]);
        put(list!["deriv", "*", 4]);

        assert_eq!(get(list!["deriv", "+"]).unwrap(), 3.to_listv());
        assert_eq!(get(list!["deriv", "*"]).unwrap(), 4.to_listv());
    }
}
