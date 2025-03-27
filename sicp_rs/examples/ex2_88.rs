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
    println!(
        "{} - {} = {}",
        pretty_polynomial(&p1, &arith),
        pretty_polynomial(&p2, &arith),
        pretty_polynomial(&arith.sub(&p1, &p2), &arith)
    );
}
