use sicp_rs::ch2::ch2_5::{
    ArithmeticContext, install_arithmetic_package, make_complex_from_real_imag, make_float,
    make_integer, make_rational, unify_arithmetic_types,
};

use sicp_rs::prelude::*;

fn main() {
    // 创建通用算术包上下文
    let arith = ArithmeticContext::new();
    install_arithmetic_package(&arith);
    // 定义测试数据
    let int1 = make_integer(5, &arith);
    let rat1 = make_rational(3.to_listv(), 4.to_listv(), &arith);
    let float1 = make_float(2.5, &arith);
    let complex1 = make_complex_from_real_imag(1.0.to_listv(), 2.0.to_listv(), &arith);

    // 测试类型提升
    let (raised_int, raised_rat) = unify_arithmetic_types(int1.clone(), rat1.clone(), &arith);
    println!(
        "{} and {} uniform to ({}, {})",
        int1, rat1, raised_int, raised_rat
    );
    println!("{} + {} = {}", int1, rat1, arith.add(&int1, &rat1));

    let (raised_rat, raised_float) = unify_arithmetic_types(rat1.clone(), float1.clone(), &arith);
    println!(
        "{} and {} uniform to ({}, {})",
        rat1, float1, raised_rat, raised_float
    );
    println!("{} + {} = {}", rat1, float1, arith.add(&rat1, &float1));

    let (raised_float, raised_complex) =
        unify_arithmetic_types(float1.clone(), complex1.clone(), &arith);
    println!(
        "{} and {} uniform to ({}, {})",
        float1, complex1, raised_float, raised_complex
    );
    println!(
        "{} + {} = {}",
        float1,
        complex1,
        arith.add(&float1, &complex1)
    );
}
