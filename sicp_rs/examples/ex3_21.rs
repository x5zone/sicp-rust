use sicp_rs::ch3::ch3_3::{delete_queue, front_ptr, insert_queue, make_queue};
use sicp_rs::prelude::*;

// pub fn delete_queue(queue: &List) -> List {
//     if is_empty_queue(queue) {
//         panic_with_location("delete_queue called with an empty queue", queue)
//     } else {
//         set_front_ptr(queue, front_ptr(queue).tail());
//         queue.clone()
//     }
// }
fn print_queue(q: &List) {
    println!("{}", front_ptr(q))
}
fn main() {
    let q1 = make_queue();
    insert_queue(&q1, "a".to_listv());
    println!("{}", q1);
    insert_queue(&q1, "b".to_listv());
    println!("{}", q1);
    delete_queue(&q1);
    println!("{}", q1);
    delete_queue(&q1);
    println!("{}", q1);
    print_queue(&q1);
}
