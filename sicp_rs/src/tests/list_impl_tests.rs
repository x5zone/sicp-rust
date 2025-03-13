use sicp_rs::{list, v, List};

#[test]
fn test_list_creation() {
    // 测试创建空链表
    let empty_list = list![];
    assert!(empty_list.is_empty());

    // 测试创建单值链表
    let single_value_list = list![v![42]];
    assert!(single_value_list.is_value());
    assert_eq!(single_value_list.length(), 1);

    // 测试创建多值链表
    let multi_value_list = list![v![1], v![2], v![3]];
    assert_eq!(multi_value_list.length(), 3);
}

#[test]
fn test_list_head_and_tail() {
    // 创建一个链表
    let l = list![v![1], v![2], v![3]];

    // 测试获取头部
    let head = l.head();
    assert!(head.is_value());
    assert_eq!(head.try_as_basis_value::<i32>().unwrap(), 1);

    // 测试获取尾部
    let tail = l.tail();
    assert_eq!(tail.length(), 2);

    // 测试尾部的头部
    let tail_head = tail.head();
    assert!(tail_head.is_value());
    assert_eq!(tail_head.try_as_basis_value::<i32>().unwrap(), 2);
}

#[test]
fn test_list_append() {
    // 创建两个链表
    let l1 = list![v![1], v![2]];
    let l2 = list![v![3], v![4]];

    // 测试链表追加
    let appended = l1.append(&l2);
    assert_eq!(appended.length(), 4);

    // 验证追加结果
    let expected = list![v![1], v![2], v![3], v![4]];
    assert_eq!(appended, expected);
}

#[test]
fn test_list_reverse() {
    // 创建一个链表
    let l = list![v![1], v![2], v![3]];

    // 测试反转
    let reversed = l.reverse();
    let expected = list![v![3], v![2], v![1]];
    assert_eq!(reversed, expected);
}

#[test]
fn test_list_deep_reverse() {
    // 创建一个嵌套链表
    let nested_list = list![v![1], list![v![2], v![3]]];

    // 测试深度反转
    let deep_reversed = nested_list.deep_reverse();
    let expected = list![list![v![3], v![2]], v![1]];
    assert_eq!(deep_reversed, expected);
}

#[test]
fn test_list_mapping() {
    // 创建一个链表
    let l = list![v![1], v![2], v![3]];

    // 测试映射功能
    let mapped = l.map(|x| {
        let value = x.try_as_basis_value::<i32>().unwrap();
        List::wrap_as_list_value(value * 2)
    });

    let expected = list![v![2], v![4], v![6]];
    assert_eq!(mapped, expected);
}

#[test]
fn test_list_filter() {
    // 创建一个链表
    let l = list![v![1], v![2], v![3], v![4]];

    // 测试过滤功能
    let filtered = l.filter(|x| {
        let value = x.try_as_basis_value::<i32>().unwrap();
        value % 2 == 0
    });

    let expected = list![v![2], v![4]];
    assert_eq!(filtered, expected);
}

#[test]
fn test_list_flatmap() {
    // 创建一个链表
    let l = list![v![1], v![2]];

    // 测试 flatmap 功能
    let flatmapped = l.flatmap(|x| {
        let value = x.try_as_basis_value::<i32>().unwrap();
        list![v![value], v![value * 10]]
    });

    let expected = list![v![1], v![10], v![2], v![20]];
    assert_eq!(flatmapped, expected);
}

#[test]
fn test_list_length() {
    // 创建一个链表
    let l = list![v![1], v![2], v![3]];

    // 测试链表长度
    assert_eq!(l.length(), 3);

    // 创建嵌套链表
    let nested_list = list![v![1], list![v![2], v![3]]];

    // 测试深度长度
    assert_eq!(nested_list.deep_length(), 3);
}

#[test]
fn test_list_fold_left() {
    // 创建一个链表
    let l = list![v![1], v![2], v![3]];

    // 测试 fold_left
    let sum = l.fold_left(|acc, x| {
        let value = x.try_as_basis_value::<i32>().unwrap();
        acc + value
    }, 0);

    assert_eq!(sum, 6);
}

#[test]
fn test_list_display() {
    // 创建一个链表
    let l = list![v![1], v![2], v![3]];

    // 测试 Display 实现
    let display = format!("{}", l);
    assert_eq!(display, "(V(1), (V(2), (V(3), Nil)))");
}
