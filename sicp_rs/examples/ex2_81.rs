use sicp_rs::{
    ch2::ch2_5::{
        ArithmeticContext, apply_generic, install_complex_package, install_float_package,
        install_integer_package, install_polar_package, install_rational_package,
        install_rectangular_package, make_complex_from_real_imag, make_float,
    },
    prelude::*,
};

fn exp(x: &List, y: &List, arith: &ArithmeticContext) -> List {
    apply_generic(&"exp".to_listv(), &list![x.clone(), y.clone()], &arith).unwrap()
}
fn main() {
    // 创建通用算术包上下文
    let mut arith = ArithmeticContext::new();
    install_integer_package(&arith);
    install_float_package(&arith);
    install_rational_package(&arith);
    install_polar_package(&arith);
    install_rectangular_package(&arith);
    install_complex_package(&arith);

    let float_to_complex_rectangular = ClosureWrapper::new({
        let arith = arith.clone();
        move |args: &List| {
            let real = args.head();
            Some(make_complex_from_real_imag(real, 0.0.to_listv(), &arith))
        }
    });

    arith.put_coercion(
        &"float".to_listv(),
        &"complex".to_listv(),
        float_to_complex_rectangular,
    );
    let c = make_complex_from_real_imag(4.0.to_listv(), 3.0.to_listv(), &arith);
    let n = make_float(7.0, &arith);
    println!("{} + {} = {}", c, n, arith.add(&c, &n));

    let complex_to_complex = ClosureWrapper::new(move |args: &List| Some(args.head()));
    arith.put_coercion(
        &"complex".to_listv(),
        &"complex".to_listv(),
        complex_to_complex,
    );
    arith.put(
        &"exp",
        list!["float", "float"],
        ClosureWrapper::new({
            let arith = arith.clone();
            move |args: &List| {
                let (base, exp) = (args.head(), args.tail().head());
                if base.is_float_value() && exp.is_float_value() {
                    let base = base.try_as_basis_value::<f64>().unwrap();
                    let exp = exp.try_as_basis_value::<f64>().unwrap();
                    Some(make_float(base.powf(*exp), &arith))
                } else {
                    panic!("Now only support f64")
                }
            }
        }),
    );

    println!("{}", exp(&10.0.to_listv(), &2.0.to_listv(), &arith));
    // panic: ArithmeticContext get failed! No func found for keys:(exp, ((complex, (complex, Nil)), Nil))
    // println!("{}", exp(&c, &c, &arith));
}
