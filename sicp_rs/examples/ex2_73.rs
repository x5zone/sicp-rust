use std::rc::Rc;

use num::Num;
use sicp_rs::ch2::ch2_3::{
    addend, augend, is_number, is_same_variable, is_variable, multiplicand, multiplier,
};
use sicp_rs::ch3::ch3_3::make_table_2d;
use sicp_rs::prelude::*;

fn operator(exp: &List) -> List {
    exp.head()
}
fn operands(exp: &List) -> List {
    exp.tail()
}
fn deriv(exp: &List, variable: &List, optable: &impl Fn(List) -> Option<List>) -> List {
    if is_number(exp) {
        list![0]
    } else if is_variable(exp) {
        if is_same_variable(exp, variable) {
            list![1]
        } else {
            list![0]
        }
    } else {
        // let op = optable.get(&"deriv".to_listv(), &operator(exp));
        // if let Some(op) = op {
        //     //let op = op.try_as_basis_value::<ClosureWrapper<Fn()>>();
        //     println!("{:?}", op);
        //     //op(&operands(exp), variable, optable)
        //     todo!()
        // } else {
        //     panic!("unknown operator -- DERIV, exp {}", exp)
        // }
        panic!("unknown operator -- DERIV, exp {}", exp)
    }
}
fn make_sum<T: Num + Clone + std::fmt::Debug + 'static>(a1: List, a2: List) -> List {
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
fn make_sum_of_deriv<T: Num + Clone + std::fmt::Debug + 'static>(
    exp: &List,
    variable: &List,
    optable: impl Fn(List) -> Option<List>,
) -> List {
    println!("make_sum_of_deriv, exp: {}", exp);
    panic!(
        "------------------------------------------------------------------------------------------------"
    );
    let a1 = deriv(&addend(exp), variable, &optable);
    let a2 = deriv(&augend(exp), variable, &optable);
    make_sum::<T>(a1, a2)
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
fn make_product_of_deriv<T: Num + Clone + std::fmt::Debug + 'static>(
    exp: &List,
    variable: &List,
    optable: impl Fn(List) -> Option<List>,
) -> List {
    let m1 = multiplier(exp);
    let m2 = deriv(&multiplicand(exp), variable, &optable);
    let a1 = make_product::<T>(m1.clone(), m2.clone());
    let m1 = deriv(&multiplier(exp), variable, &optable);
    let m2 = multiplicand(exp);
    let a2 = make_product::<T>(m1, m2);
    make_sum::<T>(a1, a2)
}

fn main() {
    let optable: Rc<dyn Fn(&str) -> ClosureWrapper> = make_table_2d();

    let op_cloned = optable.clone();
    let sum_closure = move |args: &List| {
        let a1 = args.head();
        let a2 = args.tail().head();

        let get = |args: List| op_cloned("lookup").call(&args);

        Some(make_sum_of_deriv::<i32>(&a1, &a2, &get))
    };
    let op_cloned = optable.clone();
    let product_closure = move |args: &List| {
        let a1 = args.head();
        let a2 = args.tail().head();
        let get = |args: List| op_cloned("lookup").call(&args);

        Some(make_product_of_deriv::<i32>(&a1, &a2, &get))
    };
    let get = |args: List| optable("lookup").call(&args);
    let put = |args: List| optable("insert").call(&args);
    put(list!["deriv", "+", ClosureWrapper::new(sum_closure)]);
    put(list!["deriv", "*", ClosureWrapper::new(product_closure)]);

    let op = get(list!["deriv", "+"]).unwrap();
    println!("{:?}", op);
    println!("{:?}", op.try_as_basis_value::<ClosureWrapper>()); // Output: V(A closure that satisfies the Debug trait
    //let op = op.try_as_basis_value::<ClosureWrapper<fn(&List, &List, &Table2d) -> List>>();
    let op = op.get_basis_value();
    println!("{:?}", op); // Output: V(A closure that satisfies the Debug trait)
    let op = op.as_ref();
    println!("{:?}", op); // A closure that satisfies the Debug trait
    let op = op.as_any();
    println!("{:?}", op); // A closure that satisfies the Debug trait
    let op = op.downcast_ref::<ClosureWrapper>();
    println!("{:?}", op); // None
    let op = op.unwrap();
    println!("{:?}", op); // A closure that satisfies the Debug trait
    let op = op.call(&list!["a", "b"]);
    println!("{:?}", op); // A closure that satisfies the Debug trait
}
