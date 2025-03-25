use sicp_rs::ch2::ch2_5::{ArithmeticContext, install_float_package, make_float};
fn main() {
    // 创建通用算术包上下文
    let arith = ArithmeticContext::new();
    install_float_package(&arith);

    let (x, y) = (make_float(1.0, &arith), make_float(2.0, &arith));
    println!("{} + {} = {}", x, y, arith.add(&x, &y));
}
