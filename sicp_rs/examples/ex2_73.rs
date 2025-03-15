use std::rc::Rc;

use num::Num;
use num::pow::Pow;
use sicp_rs::ch2::ch2_3::{
    addend, augend, base, exponent, is_number, is_same_variable, is_variable, make_exp,
    make_product, make_sum, multiplicand, multiplier,
};
use sicp_rs::ch3::ch3_3::make_table_2d;
use sicp_rs::prelude::*;

fn operator(exp: &List) -> List {
    exp.head()
}
fn operands(exp: &List) -> List {
    exp.tail()
}
fn deriv<T: Num + Clone + std::fmt::Debug + 'static>(
    exp: &List,
    variable: &List,
    optable: &impl Fn(List) -> Option<List>,
) -> List {
    if is_number(exp) {
        T::zero().to_listv()
    } else if is_variable(exp) {
        if is_same_variable(exp, variable) {
            T::one().to_listv()
        } else {
            T::zero().to_listv()
        }
    } else {
        let op = optable(list!["deriv", operator(exp).clone()]);
        if let Some(op) = op {
            let op = op.try_as_basis_value::<ClosureWrapper>();
            if let Ok(op) = op {
                //let result = op.call(&list![operands(exp), variable.clone()]);
                //ch2.56节代码中直接对exp取操作数,故若使用历史代码并同时使用operands,会多取一次tail
                let result = op.call(&list![exp.clone(), variable.clone()]);
                if let Some(result) = result {
                    return result;
                }
            }
        }
        panic!("unknown operator -- DERIV, exp {}", exp)
    }
}

fn make_sum_of_deriv<T: Num + Clone + std::fmt::Debug + 'static>(
    exp: &List,
    variable: &List,
    optable: impl Fn(List) -> Option<List>,
) -> List {
    let a1 = deriv::<T>(&addend(exp), variable, &optable);
    let a2 = deriv::<T>(&augend(exp), variable, &optable);
    make_sum::<T>(a1, a2)
}

fn make_product_of_deriv<T: Num + Clone + std::fmt::Debug + 'static>(
    exp: &List,
    variable: &List,
    optable: impl Fn(List) -> Option<List>,
) -> List {
    let m1 = multiplier(exp);
    let m2 = deriv::<T>(&multiplicand(exp), variable, &optable);
    let a1 = make_product::<T>(m1.clone(), m2.clone());
    let m1 = deriv::<T>(&multiplier(exp), variable, &optable);
    let m2 = multiplicand(exp);
    let a2 = make_product::<T>(m1.clone(), m2.clone());
    make_sum::<T>(a1, a2)
}
fn make_exp_of_deriv<T: Num + Clone + std::fmt::Debug + Pow<T, Output = T> + 'static>(
    exp: &List,
    variable: &List,
    optable: impl Fn(List) -> Option<List>,
) -> List {
    let base = base(exp);
    let base_cloned = base.clone();
    let exponent = exponent(exp);
    let exp_1 = make_sum::<T>(exponent.clone(), (T::zero() - T::one()).to_listv());
    make_product::<T>(
        make_product::<T>(exponent, make_exp::<T>(base, exp_1)),
        deriv::<T>(&base_cloned, variable, &optable),
    )
}

fn main() {
    let optable: Rc<dyn Fn(&str) -> ClosureWrapper> = make_table_2d();
    let op_cloned = optable.clone();
    let get = move |args: List| optable("lookup").call(&args);
    let put = move |args: List| op_cloned("insert").call(&args);

    // install deriv sum func
    let get_cloned = get.clone();
    let sum_closure = move |args: &List| {
        let a1 = args.head();
        let a2 = args.tail().head();

        Some(make_sum_of_deriv::<f64>(&a1, &a2, &get_cloned))
    };
    put(list!["deriv", "+", ClosureWrapper::new(sum_closure)]);

    // install deriv product func
    let get_cloned = get.clone();
    let product_closure = move |args: &List| {
        let a1 = args.head();
        let a2 = args.tail().head();

        Some(make_product_of_deriv::<f64>(&a1, &a2, &get_cloned))
    };
    put(list!["deriv", "*", ClosureWrapper::new(product_closure)]);

    // install deriv exp func
    let get_cloned = get.clone();
    let exp_closure = move |args: &List| {
        let a1 = args.head();
        let a2 = args.tail().head();
        Some(make_exp_of_deriv::<f64>(&a1, &a2, &get_cloned))
    };
    put(list!["deriv", "**", ClosureWrapper::new(exp_closure)]);
    // test deriv sum&product
    let exp = list!("*", list!("*", "x", "y"), list!("+", "x", 4.0));
    let get_cloned = get.clone();
    println!("{}",deriv::<f64>(&exp, &"x".to_listv(), &get_cloned).pretty_print());
    // test deriv exp
    let exp = list!("**", "x", list!("+", "y", 3.0));
    let get_cloned = get.clone();
    println!("{}",deriv::<f64>(&exp, &"x".to_listv(), &get_cloned).pretty_print());
    let exp = list!("**", "x", "n");
    let get_cloned = get.clone();
    println!("{}",deriv::<f64>(&exp, &"x".to_listv(), &get_cloned).pretty_print());
}
