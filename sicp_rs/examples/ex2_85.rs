use sicp_rs::{
    ch2::ch2_5::{
        ArithmeticContext, install_arithmetic_package, make_complex_from_real_imag, make_float,
        make_integer, make_rational,
    },
    prelude::*,
};

fn main() {
    // 创建通用算术包上下文
    let arith = ArithmeticContext::new();
    install_arithmetic_package(&arith);

    // 创建测试数据
    let complex1 = make_complex_from_real_imag(1.5.to_listv(), 0.0.to_listv(), &arith);
    let complex2 = make_complex_from_real_imag(1.3.to_listv(), 0.0.to_listv(), &arith);
    let complex3 = make_complex_from_real_imag(
        0.333333333333333333333333333.to_listv(),
        0.0.to_listv(),
        &arith,
    );
    let complex4 = make_complex_from_real_imag(1.0.to_listv(), 0.0.to_listv(), &arith);
    let complex5 = make_complex_from_real_imag(2.0.to_listv(), 3.0.to_listv(), &arith);

    // 测试 drop 函数
    println!("Original complex1: {}", complex1);
    println!("Dropped complex1: {}", arith.drop(&complex1));
    println!("Original complex2: {}", complex2);
    println!("Dropped complex2: {}", arith.drop(&complex2));
    println!("Original complex3: {}", complex3);
    println!("Dropped complex3: {}", arith.drop(&complex3));
    println!("Original complex4: {}", complex4);
    println!("Dropped complex4: {}", arith.drop(&complex4));
    println!("Original complex5: {}", complex5);
    println!("Dropped complex5: {}", arith.drop(&complex5));

    let int1 = make_integer(5, &arith);
    let rat1 = make_rational(3.to_listv(), 4.to_listv(), &arith);
    let float1 = make_float(2.5, &arith);

    let args = list![int1.clone(), rat1.clone()];
    println!(
        "result of adding {} and {}: drop to {}",
        int1,
        rat1,
        arith.drop(&arith.apply_generic("add", &args).unwrap())
    );

    let args = list![rat1.clone(), float1.clone()];

    println!(
        "result of adding {} and {}: drop to {}",
        rat1,
        float1,
        arith.drop(&arith.apply_generic("add", &args).unwrap())
    );

    let args = list![float1.clone(), complex1.clone()];

    println!(
        "result of adding {} and {}: drop to {}",
        float1,
        complex1,
        arith.drop(&arith.apply_generic("add", &args).unwrap())
    );
}
