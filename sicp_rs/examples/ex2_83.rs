use sicp_rs::{
    ch2::ch2_5::{
        ArithmeticContext, apply_generic, install_arithmetic_package, make_float, make_integer,
        make_rational,
    },
    prelude::*,
};
fn raise(x: &List, arith: &ArithmeticContext) -> List {
    apply_generic(&"raise".to_listv(), &list![x.clone()], arith).unwrap()
}
fn main() {
    // 创建通用算术包上下文
    let arith = ArithmeticContext::new();
    install_arithmetic_package(&arith);

    // 1. 测试整数提升为有理数
    let integer = make_integer(42, &arith);
    let integer_raised = raise(&integer, &arith);
    println!("integer {} raised to rational: {}", integer, integer_raised);

    // 2. 测试有理数提升为实数
    let rational = make_rational(3.to_listv(), 4.to_listv(), &arith);
    let rational_raised = raise(&rational, &arith);
    println!("rational {} raised to real: {}", rational, rational_raised);

    // 3. 测试实数提升为复数
    let real = make_float(7.0, &arith);
    let real_raised = raise(&real, &arith);
    println!("real {} raised to complex: {}", real, real_raised);
}
