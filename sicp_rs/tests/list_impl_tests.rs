use sicp_rs::prelude::*;

#[test]
fn test_list_creation() {
    // Create a list
    let l = list![1, "hello", list![2, 3], vec![4, 5], List::Nil];
    assert_eq!(
        l.to_string(),
        "(1, (hello, ((2, (3, Nil)), ([4, 5], (Nil, Nil)))))"
    );
}

#[test]
fn test_list_length() {
    let l = list![1, 2, 3];
    assert_eq!(l.length(), 3);

    let nested = list![1, list![2, 3]];
    assert_eq!(nested.length(), 2);
    assert_eq!(nested.deep_length(), 3);
}

#[test]
fn test_list_reverse() {
    let l = list![1, 2, 3];
    let reversed = l.reverse();
    assert_eq!(reversed.to_string(), "(3, (2, (1, Nil)))");

    let nested = list![1, list![2, 3]];
    let deep_reversed = nested.deep_reverse();
    assert_eq!(deep_reversed.to_string(), "((3, (2, Nil)), (1, Nil))");
}

#[test]
fn test_list_map() {
    let l = list![1, 2, 3];
    let mapped = l.map(|x| {
        let value = x.try_as_basis_value::<i32>().unwrap();
        List::wrap_as_list_value(value * 2)
    });
    assert_eq!(mapped.to_string(), "(2, (4, (6, Nil)))");
}

#[test]
fn test_list_filter() {
    let l = list![1, 2, 3, 4, 5];
    let filtered = l.filter(|x| x.try_as_basis_value::<i32>().unwrap() % 2 == 0);
    assert_eq!(filtered.to_string(), "(2, (4, Nil))");
}

#[test]
fn test_list_fold_left() {
    let l = list![1, 2, 3, 4, 5];
    let sum = l.fold_left(|acc, x| acc + x.try_as_basis_value::<i32>().unwrap(), 0);
    assert_eq!(sum, 15);
}

#[test]
fn test_list_partial_eq() {
    let l1 = list![1, 2, 3];
    let l2 = list![1, 2, 3];
    let l3 = list![3, 2, 1];

    assert!(l1 == l2, "l1 and l2 should be equal");
    assert!(l1 != l3, "l1 and l3 should not be equal");
}

#[test]
fn test_list_append() {
    let l1 = list![1, 2];
    let l2 = list![3, 4];
    let appended = l1.append(&l2);
    assert_eq!(appended.to_string(), "(1, (2, (3, (4, Nil))))");
}

#[test]
fn test_try_as_basis_value() {
    let l = list![1, "hello", 3.14];
    assert_eq!(*l.head().try_as_basis_value::<i32>().unwrap(), 1);
    assert_eq!(
        *l.tail().head().try_as_basis_value::<&str>().unwrap(),
        "hello"
    );
    assert_eq!(
        *l.tail().tail().head().try_as_basis_value::<f64>().unwrap(),
        3.14
    );
}

#[test]
fn test_nested_list_operations() {
    let nested = list![list![1, 2], list![3, 4]];
    assert_eq!(nested.length(), 2);
    assert_eq!(nested.deep_length(), 4);

    let mapped = nested.map(|x| x.map(|y| (y.try_as_basis_value::<i32>().unwrap() * 2).to_listv()));
    assert_eq!(mapped.to_string(), "((2, (4, Nil)), ((6, (8, Nil)), Nil))");
}
