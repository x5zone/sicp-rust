#![allow(dead_code)]
use sicp_rs::list;
use sicp_rs::list_impl::List;
use sicp_rs::list_impl::Wrap;
fn main() {
    let l = list![
        1,           // 自动转为 List::V(Rc::new(1))
        "hello",     // 自动转为 List::V(Rc::new("hello"))
        list![2, 3], // 嵌套列表，直接使用 List 实例
        vec![4, 5],  // 自动转为 List::V(Rc::new(vec![4, 5]))
        List::Nil    // 直接使用 List 实例
    ];
    println!("{}", l);
}
