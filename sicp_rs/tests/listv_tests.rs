use sicp_rs::prelude::ListV;

#[test]
fn test_listv_is_string() {
    let value: Box<dyn ListV> = Box::new("hello".to_string());
    assert!(value.is_string());
    assert!(!value.is_integer());
}

#[test]
fn test_listv_is_integer() {
    let value: Box<dyn ListV> = Box::new(42);
    assert!(value.is_integer());
    assert!(!value.is_float());
}

#[test]
fn test_listv_partial_eq() {
    let a: Box<dyn ListV> = Box::new(42);
    let b: Box<dyn ListV> = Box::new(42);
    assert_eq!(&a, &b);

    let c: Box<dyn ListV> = Box::new(3.14);
    let d: Box<dyn ListV> = Box::new(3.14);
    assert_eq!(&c, &d);
}

#[test]
fn test_listv_partial_cmp() {
    let a: Box<dyn ListV> = Box::new(42);
    let b: Box<dyn ListV> = Box::new(43);
    assert!(a < b);

    let c: Box<dyn ListV> = Box::new(3.14);
    let d: Box<dyn ListV> = Box::new(2.71);
    assert!(c > d);
}
