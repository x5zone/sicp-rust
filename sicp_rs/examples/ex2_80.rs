use sicp_rs::{
    ch2::ch2_5::{
        ArithmeticContext, apply_generic, install_complex_package, install_float_package,
        install_integer_package, install_polar_package, install_rational_package,
        install_rectangular_package, make_complex_from_mag_ang, make_complex_from_real_imag,
        make_float, make_integer, make_rational,
    },
    prelude::*,
};
// 需要注意apply_generic的作用: apply_generic剥去标签，并根据标签进行分派。
fn is_equal_to_zero(x: &List, arith: &ArithmeticContext) -> List {
    apply_generic(&"is_equal_to_zero".to_listv(), &list![x.clone()], arith).unwrap()
}
fn main() {
    // 创建通用算术包上下文
    let arith = ArithmeticContext::new();
    install_integer_package(&arith);
    install_float_package(&arith);
    install_rational_package(&arith);
    install_polar_package(&arith);
    install_rectangular_package(&arith);
    install_complex_package(&arith);

    let check_equal_zero = |x| println!("{} == 0.0: {}", x, is_equal_to_zero(&x, &arith));

    // 验证float的equal规则

    check_equal_zero(make_float(0.0, &arith));
    check_equal_zero(make_float(1.0, &arith));
    // 验证rational的equal规则
    // rational接受整数，复数，多项式作为参数。
    assert_eq!(make_integer(1, &arith), 1.to_listv()); // make_integer返回原始整数的List包装
    let x = make_rational(0.to_listv(), 2.to_listv(), &arith);
    check_equal_zero(x);
    let x = make_rational(1.to_listv(), 2.to_listv(), &arith);
    check_equal_zero(x);

    // 验证complex的equal规则
    let x = make_complex_from_real_imag(0.0.to_listv(), 2.0.to_listv(), &arith);
    check_equal_zero(x);
    let x = make_complex_from_real_imag(0.0.to_listv(), 0.0.to_listv(), &arith);
    check_equal_zero(x);
    let x = make_complex_from_mag_ang(1.0.to_listv(), 2.0.to_listv(), &arith);
    check_equal_zero(x);
    let x = make_complex_from_mag_ang(0.0.to_listv(), 2.0.to_listv(), &arith);
    check_equal_zero(x);
}
