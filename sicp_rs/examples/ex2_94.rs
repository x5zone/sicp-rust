use sicp_rs::{
    ch2::ch2_5::{
        ArithmeticContext, install_arithmetic_package, install_dense_terms_package,
        install_polynomial_coercion, install_polynomial_package, install_sparse_terms_package,
        make_polynomial_from_sparse, make_term, make_terms_from_sparse, pretty_polynomial,
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

    println!("==== Test: Polynomial GCD ====");

    // 测试两个多项式的最大公约数
    let p1 = make_polynomial_from_sparse(
        &"x".to_listv(),
        &make_terms_from_sparse(
            &list![
                make_term(4.to_listv(), 1.to_listv()),    // x^4
                make_term(3.to_listv(), (-1).to_listv()), // -x^3
                make_term(2.to_listv(), (-2).to_listv()), // -2x^2
                make_term(1.to_listv(), 2.to_listv()),    // 2x
            ],
            &arith,
        ),
        &arith,
    );

    let p2 = make_polynomial_from_sparse(
        &"x".to_listv(),
        &make_terms_from_sparse(
            &list![
                make_term(3.to_listv(), 1.to_listv()),    // x^3
                make_term(1.to_listv(), (-1).to_listv()), // -x
            ],
            &arith,
        ),
        &arith,
    );

    println!("Polynomial p1: {}", pretty_polynomial(&p1, &arith));
    println!("Polynomial p2: {}", pretty_polynomial(&p2, &arith));
    let gcd_poly = arith.gcd(&p1, &p2);
    println!("GCD of p1 and p2: {}", pretty_polynomial(&gcd_poly, &arith));

    println!("\n==== Test: Integer and Polynomial GCD ====");

    // 测试整数与多项式的最大公约数
    let integer = 2.to_listv();

    let poly = make_polynomial_from_sparse(
        &"x".to_listv(),
        &make_terms_from_sparse(
            &list![
                make_term(2.to_listv(), 2.to_listv()), // 2x^2
                make_term(0.to_listv(), 2.to_listv()), // 2
            ],
            &arith,
        ),
        &arith,
    );

    println!("Integer: {}", integer);
    println!("Polynomial: {}", pretty_polynomial(&poly, &arith));

    let gcd_integer_poly = arith.gcd(&integer, &poly);
    println!(
        "GCD of integer and polynomial: {}",
        pretty_polynomial(&gcd_integer_poly, &arith)
    );

    println!("\n==== Test Completed ====");
}
