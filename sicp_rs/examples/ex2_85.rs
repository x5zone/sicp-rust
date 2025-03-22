use std::{i32, rc::Rc};

use sicp_rs::{
    ch2::{
        ch2_4::{apply_generic, type_tag},
        ch2_5::{
            arithmetic_type_raise, install_arithmetic_package, is_arithmetic_type,
            make_complex_from_real_imag, make_javascript_integer, make_javascript_number,
            make_rational, raise, real_part,
        },
    },
    ch3::ch3_3::make_table_2d,
    prelude::*,
};

fn install_arithmetic_project_package(optable: Rc<dyn Fn(&str) -> ClosureWrapper>) -> Option<List> {
    let op_cloned = optable.clone();
    let get = move |args: List| optable("lookup").call(&args);
    let put = move |args: List| op_cloned("insert").call(&args);
    // project complex to real
    let get_cloned = get.clone();
    put(list![
        "project",
        list!["complex"],
        ClosureWrapper::new(move |args| {
            let real = real_part(&args.head(), get_cloned.clone());
            Some(make_javascript_number(real, get_cloned.clone()))
        })
    ]);
    // project real to rational
    let get_cloned = get.clone();
    put(list![
        "project",
        list!["javascript_number"],
        ClosureWrapper::new(move |args| {
            let real = args.head();
            let real = real.try_as_basis_value::<f64>().unwrap();
            let (numer, denom) = if (real - real.round()).abs().to_listv() == 0.0.to_listv() {
                // 小数部分等于0，是整数，直接返回
                (real.round() as i32, 1)
            } else {
                let max = (1.0 / (real - real.round()).abs()).round() as f64;
                if max == (1.0 / (real - real.round()).abs()) {
                    //1.0除以小数部分，为整数
                    ((max * real).round() as i32, max.round() as i32)
                } else {
                    let numer = (((i32::MAX as f64) / real).round() * real) as i32;
                    let denom = ((i32::MAX as f64) / real).round() as i32;
                    (numer, denom)
                }
            };

            Some(make_rational(
                numer.to_listv(),
                denom.to_listv(),
                get_cloned.clone(),
            ))
        })
    ]);
    // project rational to integer
    let get_cloned = get.clone();
    put(list![
        "project",
        list!["rational"],
        ClosureWrapper::new(move |args| {
            let numer = args.head().head();
            let denom = args.head().tail();
            let numer = *numer.try_as_basis_value::<i32>().unwrap() as f64;
            let denom = *denom.try_as_basis_value::<i32>().unwrap() as f64;
            let i = (numer / denom).floor() as i32;
            Some(make_javascript_integer(i.to_listv(), get_cloned.clone()))
        })
    ]);

    Some("done".to_string().to_listv())
}
fn project(x: &List, get: impl Fn(List) -> Option<List> + 'static) -> List {
    if !is_arithmetic_type(x) || type_tag(x) == "integer".to_listv() {
        x.clone()
    } else {
        apply_generic(&"project".to_listv(), &list![x.clone()], get).unwrap()
    }
}
fn drop(x: &List, optable: Rc<dyn Fn(&str) -> ClosureWrapper>) -> List {
    let op_cloned = optable.clone();
    let get = move |args: List| optable("lookup").call(&args);
    let new_x = project(x, get.clone());
    if raise(&new_x, get.clone()) == *x {
        drop(&new_x, op_cloned)
    } else {
        x.clone()
    }
}
fn apply_generic_drop_wrapper(
    op: &List,
    args: &List,
    optable: Rc<dyn Fn(&str) -> ClosureWrapper>,
) -> Option<List> {
    let op_cloned = optable.clone();
    let get = move |args: List| optable("lookup").call(&args);
    let (a1, a2) = (args.head(), args.tail().head());
    let (a1, a2) = if is_arithmetic_type(&a1) && is_arithmetic_type(&a2) {
        arithmetic_type_raise(a1.clone(), a2.clone(), op_cloned.clone())
    } else {
        (a1.clone(), a2.clone())
    };
    let res = apply_generic(op, &list![a1, a2], get);
    if let Some(res) = res {
        if is_arithmetic_type(&res) {
            Some(drop(&res, op_cloned))
        } else {
            Some(res)
        }
    } else {
        None
    }
}
fn main() {
    let optable = make_table_2d();
    install_arithmetic_package(optable.clone());
    install_arithmetic_project_package(optable.clone());

    let op_cloned = optable.clone();
    let get = move |args: List| op_cloned("lookup").call(&args);

    // 创建测试数据
    let complex1 = make_complex_from_real_imag(1.5.to_listv(), 0.0.to_listv(), get.clone());
    let complex2 = make_complex_from_real_imag(1.3.to_listv(), 0.0.to_listv(), get.clone());
    let complex3 = make_complex_from_real_imag(
        0.333333333333333333333333333.to_listv(),
        0.0.to_listv(),
        get.clone(),
    );
    let complex4 = make_complex_from_real_imag(1.0.to_listv(), 0.0.to_listv(), get.clone());
    let complex5 = make_complex_from_real_imag(2.0.to_listv(), 3.0.to_listv(), get.clone());

    // 测试 drop 函数
    println!("Original complex1: {}", complex1);
    println!("Dropped complex1: {}", drop(&complex1, optable.clone()));
    println!("Original complex2: {}", complex2);
    println!("Dropped complex2: {}", drop(&complex2, optable.clone()));
    println!("Original complex3: {}", complex3);
    println!("Dropped complex3: {}", drop(&complex3, optable.clone()));
    println!("Original complex4: {}", complex4);
    println!("Dropped complex4: {}", drop(&complex4, optable.clone()));
    println!("Original complex5: {}", complex5);
    println!("Dropped complex5: {}", drop(&complex5, optable.clone()));

    // 测试 apply_generic_drop_wrapper 函数
    let int1 = make_javascript_integer(5.to_listv(), get.clone());
    let rat1 = make_rational(3.to_listv(), 4.to_listv(), get.clone());
    let js_num1 = make_javascript_number(2.5.to_listv(), get.clone());

    let op = "add".to_listv();
    let args = list![int1.clone(), rat1.clone()];

    let result = apply_generic_drop_wrapper(&op, &args, optable.clone());
    println!(
        "Result of adding {} and {}: {}",
        int1,
        rat1,
        result.unwrap()
    );

    let args = list![rat1.clone(), js_num1.clone()];
    let result = apply_generic_drop_wrapper(&op, &args, optable.clone());
    println!(
        "Result of adding {} and {}: {}",
        rat1,
        js_num1,
        result.unwrap()
    );

    let args = list![js_num1.clone(), complex1.clone()];
    let result = apply_generic_drop_wrapper(&op, &args, optable.clone());
    println!(
        "Result of adding {} and {}: {}",
        js_num1,
        complex1,
        result.unwrap()
    );
}
