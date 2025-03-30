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

    // 测试整数化简
    println!("==== Testing Integer Reduction ====");
    let n = 12.to_listv();
    let d = 18.to_listv();
    let reduced = arith.reduce(&n, &d);
    println!("Reduced integers: {}", reduced); // 应输出 (2, 3)
    // 测试有理数化简
    println!("\n==== Testing Rational Number Reduction ====");
    let rational = make_rational(12.to_listv(), 18.to_listv(), &arith);
    println!("Reduced rational number: {}", rational); // 应输出 (2, 3)

    // 测试多项式化简
    println!("\n==== Testing Polynomial Reduction ====");
    let p1 = make_polynomial_from_sparse(
        // x + 1
        &"x".to_listv(),
        &make_terms_from_sparse(
            &list![
                make_term(1.to_listv(), 1.to_listv()), // x^1
                make_term(0.to_listv(), 1.to_listv()), // x^0
            ],
            &arith,
        ),
        &arith,
    );

    let p2 = make_polynomial_from_sparse(
        // x^3 - 1 = (x-1)(x^2 + x + 1)
        &"x".to_listv(),
        &make_terms_from_sparse(
            &list![
                make_term(3.to_listv(), 1.to_listv()),    // x^3
                make_term(0.to_listv(), (-1).to_listv()), // -x^0
            ],
            &arith,
        ),
        &arith,
    );
    println!("Polynomial p1: {}", pretty_polynomial(&p1, &arith));
    println!("Polynomial p2: {}", pretty_polynomial(&p2, &arith));
    let reduced_poly = arith.reduce(&p1, &p2);
    println!("Reduced polynomials:");
    println!(
        "Numerator: {}",
        pretty_polynomial(&reduced_poly.head(), &arith)
    );
    println!(
        "Denominator: {}",
        pretty_polynomial(&reduced_poly.tail().head(), &arith)
    );

    // 测试多项式有理表达式化简
    println!("\n==== Testing Polynomial Rational Reduction ====");
    let p3 = make_polynomial_from_sparse(
        // x^1
        &"x".to_listv(),
        &make_terms_from_sparse(
            &list![
                make_term(1.to_listv(), 1.to_listv()), // x^1
            ],
            &arith,
        ),
        &arith,
    );

    let p4 = make_polynomial_from_sparse(
        // x^2 - 1
        &"x".to_listv(),
        &make_terms_from_sparse(
            &list![
                make_term(2.to_listv(), 1.to_listv()),    // x^2
                make_term(0.to_listv(), (-1).to_listv()), // (-1)
            ],
            &arith,
        ),
        &arith,
    );

    let rational_poly = make_rational(p3.clone(), p4.clone(), &arith);
    println!("Reduced polynomial rational:");
    println!("Polynomial p3: {}", pretty_polynomial(&p3, &arith));
    println!("Polynomial p4: {}", pretty_polynomial(&p4, &arith));
    println!(
        "Numerator: {}",
        pretty_polynomial(&arith.numer(&rational_poly), &arith)
    );
    println!(
        "Denominator: {}",
        pretty_polynomial(&arith.denom(&rational_poly), &arith)
    );

    // 测试加法
    println!("\n==== Testing Rational Polynomial Addition ====");

    let rf1 = make_rational(p1.clone(), p2.clone(), &arith);
    let rf2 = make_rational(p3.clone(), p4.clone(), &arith);

    let sum = arith.add(&rf1, &rf2);
    println!("Sum of rational polynomials:");
    println!(
        "Numerator: {}",
        pretty_polynomial(&arith.numer(&sum), &arith)
    );
    println!(
        "Denominator: {}",
        pretty_polynomial(&arith.denom(&sum), &arith)
    );

    println!("\n==== Test Completed ====");
}
