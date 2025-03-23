use std::rc::Rc;

use sicp_rs::{
    ch2::{
        ch2_4::{apply_generic, attach_tag, type_tag},
        ch2_5::{
            add, arithmetic_type_raise, drop, imag_part, install_arithmetic_package,
            is_arithmetic_type, magnitude, make_javascript_integer, make_javascript_number,
            make_rational, mul, real_part,
        },
    },
    ch3::ch3_3::make_table_2d,
    prelude::*,
};

fn sqrt(x: &List, get: impl Fn(List) -> Option<List> + 'static) -> List {
    assert!(
        is_arithmetic_type(x) && type_tag(x) != "complex".to_listv(),
        "sqrt only for (integer, rational, javascript_number)"
    );
    apply_generic(&"sqrt".to_listv(), &list![x.clone()], get).unwrap()
}
fn install_sqrt_package(optable: Rc<dyn Fn(&str) -> ClosureWrapper>) -> Option<List> {
    let op_cloned = optable.clone();
    let get = move |args: List| op_cloned("lookup").call(&args);
    let op_cloned = optable.clone();
    let put = move |args: List| op_cloned("insert").call(&args);
    // sqrt integer
    let get_cloned = get.clone();
    let op_cloned = optable.clone();
    put(list![
        "sqrt",
        list!["integer"],
        ClosureWrapper::new(move |args| {
            let x = args.head();
            let x = x.try_as_basis_value::<i32>().unwrap();
            let x = *x as f64;
            let x = make_javascript_number(x.sqrt().to_listv(), get_cloned.clone());
            let x = drop(&x, op_cloned.clone());
            // 返回值可能不是integer
            Some(x)
        })
    ]);
    // sqrt rational
    let get_cloned = get.clone();
    let op_cloned = optable.clone();
    put(list![
        "sqrt",
        list!["rational"],
        ClosureWrapper::new(move |args| {
            let (numer, denom) = (args.head().head(), args.head().tail());
            let numer = numer.try_as_basis_value::<i32>().unwrap();
            let denom = denom.try_as_basis_value::<i32>().unwrap();
            let (numer, denom) = (*numer as f64, *denom as f64);
            let x = make_javascript_number((numer / denom).sqrt().to_listv(), get_cloned.clone());
            let x = drop(&x, op_cloned.clone());
            // 返回值可能不是rational
            Some(x)
        })
    ]);
    // sqrt javascript_number
    let get_cloned = get.clone();

    put(list![
        "sqrt",
        list!["javascript_number"],
        ClosureWrapper::new(move |args| {
            let x = args.head();
            let x = x.try_as_basis_value::<f64>().unwrap();
            let x = make_javascript_number((x.sqrt()).to_listv(), get_cloned.clone());
            Some(x)
        })
    ]);
    Some("done".to_string().to_listv())
}
fn install_extend_rectangular_package(optable: Rc<dyn Fn(&str) -> ClosureWrapper>) -> Option<List> {
    let op_cloned = optable.clone();
    let get = move |args: List| op_cloned("lookup").call(&args);
    let op_cloned = optable.clone();
    let put = move |args: List| op_cloned("insert").call(&args);
    let real_part = ClosureWrapper::new(move |x: &List| Some(x.head().head()));
    let imag_part = ClosureWrapper::new(move |x: &List| Some(x.head().tail()));
    let (real_cloned, imag_cloned) = (real_part.clone(), imag_part.clone());
    let get_real_imag = move |args: &List| {
        let (r, i) = (
            real_cloned.call(args).unwrap(),
            imag_cloned.call(args).unwrap(),
        );
        (r, i)
    };
    let get_real_imag_cloned = get_real_imag.clone();
    let get_cloned = get.clone();
    let op_cloned = optable.clone();
    let magnitude = ClosureWrapper::new(move |args: &List| {
        let (r, i) = get_real_imag_cloned(args);
        let r2 = mul(&r, &r, get_cloned.clone());
        let i2 = mul(&i, &i, get_cloned.clone());
        // 类型可能不同，需raise到相同类型
        let (r2, i2) = arithmetic_type_raise(r2, i2, op_cloned.clone());
        let x = add(&r2, &i2, get_cloned.clone(), &List::Nil);
        let x = sqrt(&x, get_cloned.clone());
        Some(x)
    });

    let make_from_real_imag = |x: List, y: List| pair![x, y];

    let tag = |x| attach_tag("erectangular", &x);
    // 注意安装操作符时，若action为具体的运算，则key2为list!包裹，list中为所有参与运算的参数的类型
    put(list!["real_part", list!["erectangular"], real_part]);
    put(list!["imag_part", list!["erectangular"], imag_part]);
    put(list!["magnitude", list!["erectangular"], magnitude]);

    // 注意安装操作符时，若action为make，则key2为单值，值为具体的类型名称
    put(list![
        "make_from_extend_real_imag",
        "erectangular",
        ClosureWrapper::new(move |args: &List| {
            let (x, y) = (args.head(), args.tail().head());
            Some(tag(make_from_real_imag(x, y)))
        })
    ]);
    Some("done".to_string().to_listv())
}

fn install_extend_complex_packages(optable: Rc<dyn Fn(&str) -> ClosureWrapper>) -> Option<List> {
    let op_cloned = optable.clone();
    let get = move |args: List| optable("lookup").call(&args);
    let put = move |args: List| op_cloned("insert").call(&args);
    let get_closure = |funlist: Option<List>| {
        funlist
            .unwrap()
            .try_as_basis_value::<ClosureWrapper>()
            .unwrap()
            .clone()
    };
    let get_cloned = get.clone();
    let make_from_real_imag = move |x: List, y: List| {
        get_closure(get_cloned(list![
            "make_from_extend_real_imag",
            "erectangular"
        ]))
        .call(&list![x, y])
        .unwrap()
    };
    let tag = |x| attach_tag("ecomplex", &x);
    put(list![
        "make_from_extend_real_imag",
        "ecomplex",
        ClosureWrapper::new(move |args: &List| {
            let x = args.head();
            let y = args.tail().head();
            Some(tag(make_from_real_imag(x.clone(), y.clone())))
        })
    ]);
    let get_cloned = get.clone();
    put(list![
        "real_part",
        list!["ecomplex"],
        ClosureWrapper::new(move |args: &List| Some(real_part(args, get_cloned.clone())))
    ]);
    let get_cloned = get.clone();
    put(list![
        "imag_part",
        list!["ecomplex"],
        ClosureWrapper::new(move |args: &List| Some(imag_part(args, get_cloned.clone())))
    ]);
    let get_cloned = get.clone();
    put(list![
        "magnitude",
        list!["ecomplex"],
        ClosureWrapper::new(move |args: &List| Some(magnitude(args, get_cloned.clone())))
    ]);

    Some("done".to_string().to_listv())
}
pub fn make_extend_complex_from_real_imag(
    x: List,
    y: List,
    get: impl Fn(List) -> Option<List> + 'static,
) -> List {
    get(list!["make_from_extend_real_imag", "ecomplex"])
        .unwrap()
        .try_as_basis_value::<ClosureWrapper>()
        .unwrap()
        .call(&list![x, y])
        .unwrap()
}
fn main() {
    let optable = make_table_2d();
    install_arithmetic_package(optable.clone());
    install_extend_rectangular_package(optable.clone());
    install_extend_complex_packages(optable.clone());
    install_sqrt_package(optable.clone());
    // 获取操作表中的操作
    let op_cloned = optable.clone();
    let get = move |args: List| op_cloned("lookup").call(&args);

    // 测试 sqrt 操作
    println!("test sqrt operation...");
    let int_test = make_javascript_integer(9.to_listv(), get.clone());
    println!("sqrt(9) = {}", sqrt(&int_test, get.clone()));

    let rational_test = make_rational(4.to_listv(), 9.to_listv(), get.clone());
    println!("sqrt(4/9) = {}", sqrt(&rational_test, get.clone()));

    // 测试创建扩展复数
    println!("\ntest extended complex numbers...");
    let real_p = make_rational(3.to_listv(), 1.to_listv(), get.clone()); // 实部 3
    let imag_p = make_javascript_integer(4.to_listv(), get.clone()); // 虚部 4
    let complex = make_extend_complex_from_real_imag(real_p.clone(), imag_p.clone(), get.clone());
    println!("created complex number: {}", complex);

    // 测试复数的实部、虚部和模
    let real_result = real_part(&complex, get.clone());
    println!("real part of complex: {}", real_result);

    let imag_result = imag_part(&complex, get.clone());
    println!("imaginary part of complex: {}", imag_result);

    let magnitude_result = magnitude(&complex, get.clone());
    println!("magnitude of complex: {}", magnitude_result);
}
