use crate::prelude::*;
use crate::list_impl::panic_with_location;
use std::fmt;

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
        key.is_value() && key.get_basis_value().is_string(),
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
pub fn insert(key: &List, value: List, table: &List) -> String {
    let record = assoc(&key, &table.tail());
    if let Some(record) = record {
        record.set_tail(value);
    } else {
        table.set_tail(pair!(pair!(key.clone(), value), table.tail()))
    }
    "ok".to_string()
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
        println!("{}",t);
    }
}
pub struct Table2d {
    local_table: List,
}
impl Table2d {
    pub fn make_table_2d() -> Self {
        Self {
            local_table: list!["*table*"],
        }
    }
    fn lookup_2d(&self, key1: &List, key2: &List) -> Option<List> {
        let subtable = assoc(&key1, &self.local_table.tail());

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
    fn insert_2d(&mut self, key1: List, key2: List, value: List) -> String {
        let subtable = assoc(&key1, &self.local_table.tail());
        if let Some(subtable) = subtable {
            let record = assoc(&key2, &subtable.tail());
            if let Some(record) = record {
                record.set_tail(value);
            } else {
                subtable.set_tail(pair!(pair!(key2, value), subtable.tail()));
            }
        } else {
            self.local_table.set_tail(pair!(
                list!(key1, pair!(key2, value)),
                self.local_table.tail()
            ));
        }
        "ok".to_string()
    }
    pub fn get(&self, key1: &List, key2: &List) -> Option<List> {
        self.lookup_2d(key1, key2)
    }
    pub fn put(&mut self, key1: List, key2: List, value: List) -> String {
        self.insert_2d(key1, key2, value)
    }
}
impl fmt::Display for Table2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.local_table)
    }
}
