use sicp_rs::prelude::*;
use num::Num;
use sicp_rs::ch2::ch2_3::{is_number, is_same_variable, is_variable};
use sicp_rs::ch3::ch3_3::Table2d;

fn operator(exp: &List) -> List {
    exp.head()
}
fn operands(exp: &List) -> List {
    exp.tail()
}
fn deriv(exp: &List, variable: &List, optable: &Table2d) -> List {
    if is_number(exp) {
        list![0]
    } else if is_variable(exp) {
        if is_same_variable(exp, variable) {
            list![1]
        } else {
            list![0]
        }
    } else {
        let op = optable.get(&"deriv".to_listv(), &operator(exp));
        if let Some(op) = op {
            todo!("Op函数未实现")
            //let op = op.try_as_basis_value::<Box<dyn Fn(&List, &List, &table_2d) -> List>>().unwrap();
            //op(&operands(exp), variable, optable)
        } else {
            panic!("unknown operator -- DERIV, exp {}", exp)
        }
        //panic!("unknown operator -- DERIV, exp {}", exp)
    }
}
fn make_sum<T: Num  + Clone + std::fmt::Debug + 'static>(a1: List, a2: List) -> List {
    if a1 == T::zero().to_listv() {
        a2
    } else if a2 == T::zero().to_listv() {
        a1
    } else if is_number(&a1) && is_number(&a2) {
        (a1.try_as_basis_value::<T>().unwrap().clone()
            + a2.try_as_basis_value::<T>().unwrap().clone())
        .to_listv()
    } else {
        list!["+", a1, a2]
    }
}
fn make_product<T: Num + Clone + std::fmt::Debug + 'static>(m1: List, m2: List) -> List {
    if m1 == T::zero().to_listv() || m2 == T::zero().to_listv() {
        T::zero().to_listv()
    } else if m1 == T::one().to_listv() {
        m2
    } else if m2 == T::one().to_listv() {
        m1
    } else if is_number(&m1) && is_number(&m2) {
        return (m1.try_as_basis_value::<T>().unwrap().clone()
            * m2.try_as_basis_value::<T>().unwrap().clone())
        .to_listv();
    } else {
        list!["*", m1, m2]
    }
}
fn main() {
    let mut optable = Table2d::make_table_2d();
    optable.put("deriv".to_listv(), "+".to_listv(), 3.to_listv());
    optable.put("deriv".to_listv(), "+".to_listv(), 3.to_listv());
    optable.put("deriv".to_listv(), "*".to_listv(), 4.to_listv());
    println!("-------{}", optable);
    println!("-------{}", optable.get(&"deriv".to_listv(), &"+".to_listv()).unwrap());
}

