use sicp_rs::{
    ch2::ch2_5::{
        ArithmeticContext, install_arithmetic_package, install_dense_terms_package,
        install_polynomial_package, install_sparse_terms_package, make_empty_term_list,
        make_polynomial_from_dense, make_polynomial_from_sparse, make_term, make_terms_from_dense,
        make_terms_from_sparse, pretty_polynomial,
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

    // 测试1：创建稀疏多项式和稠密多项式
    println!("==== Test 1: Create Sparse and Dense Polynomials ====");
    let sparse_term_list = make_terms_from_sparse(
        &list![
            make_term(2.to_listv(), 4.to_listv()),
            make_term(1.to_listv(), 3.to_listv()),
            make_term(0.to_listv(), 7.to_listv())
        ],
        &arith,
    );
    let dense_term_list = make_terms_from_dense(
        &list![
            7.to_listv(), // x^0
            3.to_listv(), // x^1
            4.to_listv()  // x^2
        ],
        &arith,
    );
    let sparse_poly = make_polynomial_from_sparse(&"x".to_listv(), &sparse_term_list, &arith);
    let dense_poly = make_polynomial_from_dense(&"x".to_listv(), &dense_term_list, &arith);
    println!("Sparse Polynomial: {}", sparse_poly);
    println!(
        "Sparse Polynomial: {}",
        pretty_polynomial(&sparse_poly, &arith)
    );
    println!("Dense Polynomial: {}", dense_poly);
    println!(
        "Dense Polynomial: {}",
        pretty_polynomial(&dense_poly, &arith)
    );

    // 测试2：加法
    println!("\n==== Test 2: Polynomial Addition ====");
    let sum_poly = arith.add(&sparse_poly, &dense_poly);
    println!(
        "Sum of Sparse and Dense Polynomials: {}",
        pretty_polynomial(&sum_poly, &arith)
    );

    // 测试3：乘法
    println!("\n==== Test 3: Polynomial Multiplication ====");
    let product_poly = arith.mul(&sparse_poly, &dense_poly);
    println!(
        "Product of Sparse and Dense Polynomials: {}",
        pretty_polynomial(&product_poly, &arith)
    );

    // 测试4：减法
    println!("\n==== Test 4: Polynomial Subtraction ====");
    let diff_poly = arith.sub(&sparse_poly, &dense_poly);
    println!(
        "Difference of Sparse and Dense Polynomials: {}",
        pretty_polynomial(&diff_poly, &arith)
    );

    // 测试5：取负
    println!("\n==== Test 5: Polynomial Negation ====");
    let neg_sparse_poly = arith.negative(&sparse_poly);
    let neg_dense_poly = arith.negative(&dense_poly);
    println!(
        "Negative Sparse Polynomial: {}",
        pretty_polynomial(&neg_sparse_poly, &arith)
    );
    println!(
        "Negative Dense Polynomial: {}",
        pretty_polynomial(&neg_dense_poly, &arith)
    );

    // 测试6：混合运算 (稀疏 + 稠密)
    println!("\n==== Test 6: Mixed Operations (Sparse + Dense) ====");
    let mixed_sum = arith.add(&sparse_poly, &dense_poly);
    let mixed_product = arith.mul(&sparse_poly, &dense_poly);
    let mixed_diff = arith.sub(&sparse_poly, &dense_poly);
    println!(
        "Mixed Sum (Sparse + Dense): {}",
        pretty_polynomial(&mixed_sum, &arith)
    );
    println!(
        "Mixed Product (Sparse * Dense): {}",
        pretty_polynomial(&mixed_product, &arith)
    );
    println!(
        "Mixed Difference (Sparse - Dense): {}",
        pretty_polynomial(&mixed_diff, &arith)
    );

    // 测试7：零多项式判定
    println!("\n==== Test 7: Zero Polynomial Detection ====");
    let zero_poly =
        make_polynomial_from_sparse(&"x".to_listv(), &make_empty_term_list(&arith), &arith);
    println!("Zero Polynomial: {}", pretty_polynomial(&zero_poly, &arith));
    println!("Is Zero Polynomial: {}", arith.is_equal_to_zero(&zero_poly));

    // 测试8：复杂嵌套多项式
    println!("\n==== Test 8: Nested Polynomials ====");
    let nested_poly = make_polynomial_from_sparse(
        &"y".to_listv(),
        &make_terms_from_sparse(
            &list![
                make_term(1.to_listv(), sparse_poly.clone()),
                make_term(0.to_listv(), dense_poly.clone())
            ],
            &arith,
        ),
        &arith,
    );
    println!(
        "Nested Polynomial: {}",
        pretty_polynomial(&nested_poly, &arith)
    );

    println!("\n==== All Tests Completed Successfully ====");
}
