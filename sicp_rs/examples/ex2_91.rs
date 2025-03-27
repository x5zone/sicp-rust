use sicp_rs::{
    ch2::ch2_5::{
        ArithmeticContext, install_arithmetic_package, install_dense_terms_package,
        install_polynomial_package, install_sparse_terms_package, make_empty_term_list,
        make_polynomial_from_sparse, make_term, make_terms_from_sparse, pretty_polynomial,
    },
    prelude::*,
};

fn main() {
    // 初始化ArithmeticContext
    let arith = ArithmeticContext::new();
    install_arithmetic_package(&arith);
    install_sparse_terms_package(&arith);
    install_dense_terms_package(&arith);
    install_polynomial_package(&arith);

    // 测试1：多项式除法
    println!("==== Test 1: Polynomial Division ====");
    let dividend_sparse = make_polynomial_from_sparse(
        &"x".to_listv(),
        &make_terms_from_sparse(
            &list![
                make_term(5.to_listv(), 1.to_listv()),    // x^5
                make_term(0.to_listv(), (-1).to_listv())  // -1
            ],
            &arith,
        ),
        &arith,
    );

    let divisor_sparse = make_polynomial_from_sparse(
        &"x".to_listv(),
        &make_terms_from_sparse(
            &list![
                make_term(2.to_listv(), 1.to_listv()),    // x^2
                make_term(0.to_listv(), (-1).to_listv())  // -1
            ],
            &arith,
        ),
        &arith,
    );

    println!(
        "Dividend Polynomial: {}",
        pretty_polynomial(&dividend_sparse, &arith)
    );
    println!(
        "Divisor Polynomial: {}",
        pretty_polynomial(&divisor_sparse, &arith)
    );

    let result = arith.div(&dividend_sparse, &divisor_sparse);
    let quotient = result.head(); // 商式
    let remainder = result.tail().head(); // 余式
    println!(
        "Quotient Polynomial: {}",
        pretty_polynomial(&quotient, &arith)
    );
    println!(
        "Remainder Polynomial: {}",
        pretty_polynomial(&remainder, &arith)
    );

    // 验证：商和余式的正确性
    let reconstructed_dividend = arith.add(&arith.mul(&quotient, &divisor_sparse), &remainder);

    println!(
        "Reconstructed Dividend (Quotient * Divisor + Remainder): {}",
        pretty_polynomial(&reconstructed_dividend, &arith)
    );
    println!("{}\n{}", reconstructed_dividend, dividend_sparse);
    println!(
        "Is Reconstruction Equal to Original Dividend: {}",
        arith.is_equal(&reconstructed_dividend, &dividend_sparse)
    );

    // 测试2：零多项式除法
    println!("\n==== Test 2: Zero Polynomial Division ====");
    let zero_poly =
        make_polynomial_from_sparse(&"x".to_listv(), &make_empty_term_list(&arith), &arith);

    let zero_div_result = arith.div(&zero_poly, &divisor_sparse);
    let zero_quotient = zero_div_result.head();
    let zero_remainder = zero_div_result.tail().head();

    println!(
        "Zero Polynomial Quotient: {}",
        pretty_polynomial(&zero_quotient, &arith)
    );
    println!(
        "Zero Polynomial Remainder: {}",
        pretty_polynomial(&zero_remainder, &arith)
    );

    println!("\n==== All Tests Completed Successfully ====");
}
