use sicp_rs::{
    ch2::ch2_5::{
        ArithmeticContext, install_arithmetic_package, install_dense_terms_package,
        install_polynomial_coercion, install_polynomial_package, install_sparse_terms_package,
        make_polynomial_from_sparse, make_term, make_terms_from_dense, make_terms_from_sparse,
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

    println!("==== Test: Polynomial GCD ====");

    // 测试两个多项式的最大公约数
    let p1 = make_polynomial_from_sparse(
        &"x".to_listv(),
        &make_terms_from_dense(&list![1, -2, 1], &arith), // 1*x^2 + (-2)*x^1 + 1*x^0
        &arith,
    );

    let p2 = make_polynomial_from_sparse(
        &"x".to_listv(),
        &make_terms_from_sparse(
            &list![
                make_term(2.to_listv(), 11.to_listv()),  // 11*x^2
                make_term(0.to_listv(), (7).to_listv()), // 7*x^0
            ],
            &arith,
        ),
        &arith,
    );
    let p3 = make_polynomial_from_sparse(
        &"x".to_listv(),
        &make_terms_from_sparse(
            &list![
                make_term(1.to_listv(), 13.to_listv()),  // 13*x^1
                make_term(0.to_listv(), (5).to_listv()), // 5*x^0
            ],
            &arith,
        ),
        &arith,
    );
    let q1 = arith.mul(&p1, &p2);
    let q2 = arith.mul(&p1, &p3);
    println!("Polynomial q1: {}", pretty_polynomial(&q1, &arith));
    println!("Polynomial q2: {}", pretty_polynomial(&q2, &arith));
    let gcd_poly = arith.gcd(&q1, &q2);
    println!("GCD of q1 and q2: {}", pretty_polynomial(&gcd_poly, &arith));

    println!("\n==== Test Completed ====");
}
