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

    println!("==== Test 1: Polynomial Addition with Different Variables ====");
    // 多项式1：x^2 + 2x + 1
    let poly1 = make_polynomial_from_sparse(
        &"x".to_listv(),
        &make_terms_from_sparse(
            &list![
                make_term(2.to_listv(), 1.to_listv()), // x^2
                make_term(1.to_listv(), 2.to_listv()), // 2x^1
                make_term(0.to_listv(), 1.to_listv()), // 1
            ],
            &arith,
        ),
        &arith,
    );
    // 多项式2：y^3 + y
    let poly2 = make_polynomial_from_sparse(
        &"y".to_listv(),
        &make_terms_from_sparse(
            &list![
                make_term(3.to_listv(), 1.to_listv()), // y^3
                make_term(1.to_listv(), 1.to_listv()), // y^1
            ],
            &arith,
        ),
        &arith,
    );

    println!("Polynomial 1: {}", pretty_polynomial(&poly1, &arith));
    println!("Polynomial 2: {}", pretty_polynomial(&poly2, &arith));
    // 多项式加法
    let add_result = arith.add(&poly1, &poly2);
    println!(
        "Addition Result: {}",
        pretty_polynomial(&add_result, &arith)
    );

    println!("\n==== Test 2: Polynomial Multiplication with Different Variables ====");
    // 多项式3：z^2 + 3z + 2
    let poly3 = make_polynomial_from_sparse(
        &"z".to_listv(),
        &make_terms_from_sparse(
            &list![
                make_term(2.to_listv(), 1.to_listv()), // z^2
                make_term(1.to_listv(), 3.to_listv()), // 3z^1
                make_term(0.to_listv(), 2.to_listv()), // 2
            ],
            &arith,
        ),
        &arith,
    );

    println!("Polynomial 3: {}", pretty_polynomial(&poly3, &arith));

    // 多项式乘法
    let mul_result = arith.mul(&poly1, &poly3);
    println!(
        "Multiplication Result: {}",
        pretty_polynomial(&mul_result, &arith)
    );

    println!("\n==== Test 3: Coercion of Numbers to Polynomials ====");
    // 测试 num 转换为多项式
    let num = 5.to_listv();
    let coerced_poly_func = arith
        .get_coercion(&"integer".to_listv(), &"polynomial".to_listv())
        .unwrap();
    let coerced_poly = coerced_poly_func.call(&list![num.clone()]).unwrap();
    println!("Number: {}", num);
    println!(
        "Coerced Polynomial: {}, pretty print {}",
        coerced_poly.pretty_print(),
        pretty_polynomial(&coerced_poly, &arith)
    );

    println!("\n==== Test 4: Polynomial with Nested Coefficients ====");
    // 嵌套多项式：x^2 + (y^2 + y)*x + (y^3 + y)
    let nested_coeff = make_polynomial_from_sparse(
        &"y".to_listv(),
        &make_terms_from_sparse(
            &list![
                make_term(2.to_listv(), 1.to_listv()), // y^2
                make_term(1.to_listv(), 1.to_listv()), // y^1
            ],
            &arith,
        ),
        &arith,
    );

    let nested_poly = make_polynomial_from_sparse(
        &"x".to_listv(),
        &make_terms_from_sparse(
            &list![
                make_term(2.to_listv(), 1.to_listv()),         // x^2
                make_term(1.to_listv(), nested_coeff.clone()), // (y^2 + y)*x
                make_term(0.to_listv(), nested_coeff),         // y^3 + y
            ],
            &arith,
        ),
        &arith,
    );

    println!(
        "Nested Polynomial: {}",
        pretty_polynomial(&nested_poly, &arith)
    );
    println!("\n==== Test 5: Rational Function Operations ====");
    // 有理函数1： (x + 1) / (x^3 - 1)
    let numerator1 = make_polynomial_from_sparse(
        &"x".to_listv(),
        &make_terms_from_sparse(
            &list![
                make_term(1.to_listv(), 1.to_listv()), // x
                make_term(0.to_listv(), 1.to_listv()), // 1
            ],
            &arith,
        ),
        &arith,
    );

    let denominator1 = make_polynomial_from_sparse(
        &"x".to_listv(),
        &make_terms_from_sparse(
            &list![
                make_term(3.to_listv(), 1.to_listv()),    // x^3
                make_term(0.to_listv(), (-1).to_listv()), // -1
            ],
            &arith,
        ),
        &arith,
    );

    let rational1 = make_rational(numerator1.clone(), denominator1.clone(), &arith);

    // 有理函数2： (x^2 + 2x^2 + 3x + 1) / (x^3 - x - 1)
    let numerator2 = make_polynomial_from_sparse(
        &"x".to_listv(),
        &make_terms_from_sparse(
            &list![
                make_term(2.to_listv(), 2.to_listv()), // 2x^2
                make_term(1.to_listv(), 3.to_listv()), // 3x
                make_term(0.to_listv(), 1.to_listv()), // 1
            ],
            &arith,
        ),
        &arith,
    );

    let denominator2 = make_polynomial_from_sparse(
        &"x".to_listv(),
        &make_terms_from_sparse(
            &list![
                make_term(3.to_listv(), 1.to_listv()),    // x^3
                make_term(1.to_listv(), (-1).to_listv()), // -x
                make_term(0.to_listv(), (-1).to_listv()), // -1
            ],
            &arith,
        ),
        &arith,
    );

    let rational2 = make_rational(numerator2.clone(), denominator2.clone(), &arith);

    println!("Rational Function 1:");
    println!("  Numerator: {}", pretty_polynomial(&arith.numer(&rational1), &arith));
    println!("  Denominator: {}", pretty_polynomial(&arith.denom(&rational1), &arith));

    println!("Rational Function 2:");
    println!("  Numerator: {}", pretty_polynomial(&arith.numer(&rational2), &arith));
    println!("  Denominator: {}", pretty_polynomial(&arith.denom(&rational2), &arith));

    // 有理函数加法
    let rational_add = arith.add(&rational1, &rational2);
    println!("Rational Addition Result:");
    println!(
        "  Numerator: {}",
        pretty_polynomial(&arith.numer(&rational_add), &arith)
    );
    println!(
        "  Denominator: {}",
        pretty_polynomial(&arith.denom(&rational_add), &arith)
    );

    // 有理函数乘法
    let rational_mul = arith.mul(&rational1, &rational2);
    println!("Rational Multiplication Result:");
    println!(
        "  Numerator: {}",
        pretty_polynomial(&arith.numer(&rational_mul), &arith)
    );
    println!(
        "  Denominator: {}",
        pretty_polynomial(&arith.denom(&rational_mul), &arith)
    );

    println!("\n==== All Tests Completed Successfully ====");
}
