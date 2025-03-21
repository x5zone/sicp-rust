use std::rc::Rc;

use sicp_rs::{
    ch2::{
        ch2_4::apply_generic,
        ch2_5::{
            install_complex_packages, install_javascript_integer_package,
            install_javascript_number_package, install_polar_package, install_rational_package,
            install_rectangular_package, make_complex_from_real_imag, make_javascript_integer,
            make_javascript_number, make_rational,
        },
    },
    ch3::ch3_3::make_table_2d,
    prelude::*,
};
fn install_integer_raise_package(optable: Rc<dyn Fn(&str) -> ClosureWrapper>) -> Option<List> {
    let op_cloned = optable.clone();
    let get = move |args: List| optable("lookup").call(&args);
    let put = move |args: List| op_cloned("insert").call(&args);
    put(list![
        "raise",
        list!["integer"],
        ClosureWrapper::new(move |args| {
            let i = args.head();
            Some(make_rational(i, 1.to_listv(), get.clone()))
        })
    ]);
    Some("done".to_string().to_listv())
}
fn install_rational_raise_package(optable: Rc<dyn Fn(&str) -> ClosureWrapper>) -> Option<List> {
    let op_cloned = optable.clone();
    let get = move |args: List| optable("lookup").call(&args);
    let put = move |args: List| op_cloned("insert").call(&args);
    put(list![
        "raise",
        list!["rational"],
        ClosureWrapper::new(move |args| {
            let (numer_x, denom_x) = (args.head().head(), args.head().tail());
            let (numer_x, denom_x) = (
                numer_x
                    .try_as_basis_value::<i32>()
                    .expect("rational numerator must be i32"),
                denom_x
                    .try_as_basis_value::<i32>()
                    .expect("rational denominator must be i32"),
            );
            Some(make_javascript_number(
                ((*numer_x as f64) / (*denom_x as f64)).to_listv(),
                get.clone(),
            ))
        })
    ]);
    Some("done".to_string().to_listv())
}
fn install_javascript_number_raise_package(
    optable: Rc<dyn Fn(&str) -> ClosureWrapper>,
) -> Option<List> {
    let op_cloned = optable.clone();
    let get = move |args: List| optable("lookup").call(&args);
    let put = move |args: List| op_cloned("insert").call(&args);
    put(list![
        "raise",
        list!["javascript_number"],
        ClosureWrapper::new(move |args| {
            let i = args.head();

            Some(make_complex_from_real_imag(i, 0.0.to_listv(), get.clone()))
        })
    ]);

    Some("done".to_string().to_listv())
}
fn raise(x: &List, get: impl Fn(List) -> Option<List> + 'static) -> List {
    apply_generic(&"raise".to_listv(), &list![x.clone()], get).unwrap()
}
fn main() {
    let optable = make_table_2d();
    let op_cloned = optable.clone();
    let get = move |args: List| op_cloned("lookup").call(&args);
    let op_cloned = optable.clone();
    let put = move |args: List| op_cloned("insert").call(&args);
    let op_cloned = optable.clone();
    install_complex_packages(op_cloned);
    install_rectangular_package(put.clone());
    install_polar_package(put.clone());
    install_rational_package(put.clone());
    install_javascript_number_package(put.clone());
    install_javascript_integer_package(put.clone());
    install_integer_raise_package(optable.clone());
    install_rational_raise_package(optable.clone());
    install_javascript_number_raise_package(optable.clone());

    // 1. 测试整数提升为有理数
    let integer = make_javascript_integer(42.to_listv(), get.clone());
    let integer_raised = raise(&integer, get.clone());
    println!("integer {} raised to rational: {}", integer, integer_raised);

    // 2. 测试有理数提升为实数
    let rational = make_rational(3.to_listv(), 4.to_listv(), get.clone());
    let rational_raised = raise(&rational, get.clone());
    println!("rational {} raised to real: {}", rational, rational_raised);

    // 3. 测试实数提升为复数
    let real = make_javascript_number(7.0.to_listv(), get.clone());
    let real_raised = raise(&real, get.clone());
    println!("real {} raised to complex: {}", real, real_raised);
}
