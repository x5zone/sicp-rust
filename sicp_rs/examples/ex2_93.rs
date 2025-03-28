use sicp_rs::{
    ch2::ch2_5::{
        ArithmeticContext, install_arithmetic_package, install_dense_terms_package,
        install_polynomial_coercion, install_polynomial_package, install_sparse_terms_package,
        make_polynomial_from_sparse, make_rational, make_term, make_terms_from_sparse,
        pretty_polynomial,
    },
    prelude::*,
};

fn main() {
    // 初始化 ArithmeticContext
    let mut arith = ArithmeticContext::new();

    install_arithmetic_package(&arith);
    install_sparse_terms_package(&arith);
    install_dense_terms_package(&arith);
    install_polynomial_package(&arith);
    install_polynomial_coercion(&mut arith);

    println!("==== Test: Rational Function Creation and Addition ====");

    // 多项式 p1: x^2 + 1
    let p1 = make_polynomial_from_sparse(
        &"x".to_listv(),
        &make_terms_from_sparse(
            &list![
                make_term(2.to_listv(), 1.to_listv()), // x^2
                make_term(0.to_listv(), 1.to_listv()), // 1
            ],
            &arith,
        ),
        &arith,
    );

    // 多项式 p2: x^3 + 1
    let p2 = make_polynomial_from_sparse(
        &"x".to_listv(),
        &make_terms_from_sparse(
            &list![
                make_term(3.to_listv(), 1.to_listv()), // x^3
                make_term(0.to_listv(), 1.to_listv()), // 1
            ],
            &arith,
        ),
        &arith,
    );

    println!("Polynomial p1: {}", pretty_polynomial(&p1, &arith));
    println!("Polynomial p2: {}", pretty_polynomial(&p2, &arith));

    // 创建有理函数 rf = p2 / p1
    let rf = make_rational(p2.clone(), p1.clone(), &arith);
    println!("Rational Function rf:");
    println!(
        "  Numerator: {}",
        pretty_polynomial(&arith.numer(&rf), &arith)
    );
    println!(
        "  Denominator: {}",
        pretty_polynomial(&arith.denom(&rf), &arith)
    );

    // 将 rf 与自身相加
    let rf_add = arith.add(&rf, &rf);
    println!("Rational Function rf + rf:");
    println!(
        "  Numerator: {}",
        pretty_polynomial(&arith.numer(&rf_add), &arith)
    );
    println!(
        "  Denominator: {}",
        pretty_polynomial(&arith.denom(&rf_add), &arith)
    );

    println!("\n==== Test Completed ====");
}
