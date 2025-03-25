use sicp_rs::{
    ch2::ch2_5::{
        ArithmeticContext, install_arithmetic_package, make_complex_from_real_imag, make_integer,
        make_rational,
    },
    prelude::*,
};
fn main() {
    let arith = ArithmeticContext::new();
    install_arithmetic_package(&arith);

    // 测试 sqrt 操作
    println!("test sqrt operation...");
    let int_test = make_integer(9, &arith);
    println!("sqrt(9) = {}", arith.sqrt(&int_test));

    let rational_test = make_rational(4.to_listv(), 9.to_listv(), &arith);
    println!("sqrt(4/9) = {}", arith.sqrt(&rational_test));

    // 测试创建扩展复数
    println!("\ntest complex numbers...");
    let real_p = make_rational(3.to_listv(), 1.to_listv(), &arith); // 实部 3
    let imag_p = make_integer(4, &arith); // 虚部 4
    let complex = make_complex_from_real_imag(real_p.clone(), imag_p.clone(), &arith);
    println!("created complex number: {}", complex);

    // 测试复数的实部、虚部和模
    let real_result = arith.real_part(&complex);
    println!("real part of complex: {}", real_result);

    let imag_result = arith.imag_part(&complex);
    println!("imaginary part of complex: {}", imag_result);

    let magnitude_result = arith.magnitude(&complex);
    println!("magnitude of complex: {}", magnitude_result);
}
