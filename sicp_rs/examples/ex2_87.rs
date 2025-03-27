use sicp_rs::{
    ch2::ch2_5::{
        ArithmeticContext, install_arithmetic_package, install_polynomial_package,
        install_sparse_terms_package, make_float, make_integer, make_polynomial_from_sparse,
        make_term, pretty_polynomial,
    },
    prelude::*,
};

fn main() {
    let arith = ArithmeticContext::new();
    install_arithmetic_package(&arith);
    install_sparse_terms_package(&arith);
    install_polynomial_package(&arith);

    let p1 = make_polynomial_from_sparse(
        &"x".to_listv(),
        &list![
            make_term(2.to_listv(), make_integer(4, &arith)),
            make_term(1.to_listv(), make_integer(3, &arith)),
            make_term(0.to_listv(), make_float(7.0, &arith)),
        ],
        &arith,
    );
    let p2 = make_polynomial_from_sparse(
        &"x".to_listv(),
        &list![
            make_term(2.to_listv(), make_integer(5, &arith)),
            make_term(1.to_listv(), make_float(2.0, &arith)),
            make_term(0.to_listv(), make_float(10.0, &arith)),
        ],
        &arith,
    );
    println!("polynomial representation: {}\n", p1.pretty_print());

    // 测试多项式的乘法和加法
    println!("test polynomial multiplication and addition...");
    println!(
        " {} + {} = {}",
        pretty_polynomial(&p1, &arith),
        pretty_polynomial(&p2, &arith),
        pretty_polynomial(&arith.add(&p1, &p2), &arith)
    );
    println!(
        " {} * {} = {}\n",
        pretty_polynomial(&p1, &arith),
        pretty_polynomial(&p2, &arith),
        pretty_polynomial(&arith.mul(&p1, &p2), &arith)
    );
    let p3 = make_polynomial_from_sparse(
        &"x".to_listv(),
        &list![
            make_term(2.to_listv(), make_integer(5, &arith)),
            make_term(1.to_listv(), p2.clone()),
            make_term(0.to_listv(), make_float(10.0, &arith)),
        ],
        &arith,
    );
    println!("polynomial as coeff: {}\n", pretty_polynomial(&p3, &arith));
    let zero = make_polynomial_from_sparse(
        &"x".to_listv(),
        &list![
            make_term(2.to_listv(), make_integer(0, &arith)),
            make_term(1.to_listv(), make_integer(0, &arith)),
            make_term(0.to_listv(), make_float(0.0, &arith)),
        ],
        &arith,
    );
    let p3 = make_polynomial_from_sparse(
        &"x".to_listv(),
        &list![
            make_term(2.to_listv(), make_integer(5, &arith)),
            make_term(1.to_listv(), zero.clone()),
            make_term(0.to_listv(), make_float(10.0, &arith)),
        ],
        &arith,
    );
    println!(
        "zero {} is_equal_to_zero {}, as coeff: {}\n {} + {} = {}",
        pretty_polynomial(&zero, &arith),
        arith.is_equal_to_zero(&zero),
        pretty_polynomial(&p3, &arith),
        pretty_polynomial(&p3, &arith),
        pretty_polynomial(&p3, &arith),
        pretty_polynomial(&arith.add(&p3, &p3), &arith)
    );
    let zero = make_polynomial_from_sparse(&"x".to_listv(), &list![], &arith);
    let p3 = make_polynomial_from_sparse(
        &"x".to_listv(),
        &list![
            make_term(2.to_listv(), make_integer(5, &arith)),
            make_term(1.to_listv(), p2),
            make_term(0.to_listv(), make_float(10.0, &arith)),
        ],
        &arith,
    );
    println!(
        "zero {} is_equal_to_zero {}, as coeff: {}\n {} + {} = {}",
        pretty_polynomial(&zero, &arith),
        arith.is_equal_to_zero(&zero),
        pretty_polynomial(&p3, &arith),
        pretty_polynomial(&p3, &arith),
        pretty_polynomial(&p3, &arith),
        pretty_polynomial(&arith.add(&p3, &p3), &arith)
        // p3 + p1 will panic because now not support raising numbers to polynomials.
        // pretty_polynomial(&arith.add(&p3, &p1), &arith)
    );
}
