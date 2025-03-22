use std::rc::Rc;

use sicp_rs::ch2::ch2_4::type_tag;
use sicp_rs::ch2::ch2_5::{
    install_arithmetic_raise_package, install_complex_packages, install_javascript_integer_package,
    install_javascript_number_package, install_polar_package, install_rational_package,
    install_rectangular_package, make_complex_from_real_imag, make_javascript_integer,
    make_javascript_number, make_rational, raise,
};
use sicp_rs::ch3::ch3_3::make_table_2d;
use sicp_rs::prelude::*;

const ARITHMETIC_TYPES: [&str; 4] = ["integer", "rational", "javascript_number", "complex"];

fn find_index(type_tag: &str) -> i32 {
    for (i, t) in ARITHMETIC_TYPES.iter().enumerate() {
        if type_tag == *t {
            return i as i32;
        }
    }
    -1
}

fn arithmetic_type_raise(
    a1: List,
    a2: List,
    optable: Rc<dyn Fn(&str) -> ClosureWrapper>,
) -> (List, List) {
    let a1_type_tag = type_tag(&a1);
    let a2_type_tag = type_tag(&a2);
    let a1_index = find_index(&a1_type_tag.to_string());
    let a2_index = find_index(&a2_type_tag.to_string());
    let get = move |args: List| optable("lookup").call(&args);
    fn raise_helper(
        x: &List,
        index_diff: i32,
        get: impl Fn(List) -> Option<List> + 'static + Clone,
    ) -> List {
        if index_diff <= 0 {
            x.clone()
        } else {
            raise_helper(&raise(x, get.clone()), index_diff - 1, get)
        }
    }
    let a1 = if a1_index < a2_index {
        raise_helper(&a1, a2_index - a1_index, get.clone())
    } else {
        a1
    };
    let a2 = if a1_index > a2_index {
        raise_helper(&a2, a1_index - a2_index, get.clone())
    } else {
        a2
    };
    (a1, a2)
}
fn main() {
    let optable = make_table_2d();
    let op_cloned = optable.clone();
    let get = move |args: List| op_cloned("lookup").call(&args);
    let op_cloned = optable.clone();
    let put = move |args: List| op_cloned("insert").call(&args);
    let op_cloned = optable.clone();
    install_complex_packages(op_cloned);
    install_rectangular_package(put.clone());
    install_polar_package(put.clone());
    install_rational_package(put.clone());
    install_javascript_number_package(put.clone());
    install_javascript_integer_package(put.clone());
    install_arithmetic_raise_package(optable.clone());
    // 定义测试数据
    let int1 = make_javascript_integer(5.to_listv(), get.clone());
    let rat1 = make_rational(3.to_listv(), 4.to_listv(), get.clone());
    let js_num1 = make_javascript_number(2.5.to_listv(), get.clone());
    let complex1 = make_complex_from_real_imag(1.0.to_listv(), 2.0.to_listv(), get.clone());

    // 测试类型提升
    let (raised_int, raised_rat) =
        arithmetic_type_raise(int1.clone(), rat1.clone(), optable.clone());
    println!("Raised int: {}, Raised rat: {}", raised_int, raised_rat);

    let (raised_rat, raised_js_num) =
        arithmetic_type_raise(rat1.clone(), js_num1.clone(), optable.clone());
    println!(
        "Raised rat: {}, Raised js_num: {}",
        raised_rat, raised_js_num
    );

    let (raised_js_num, raised_complex) =
        arithmetic_type_raise(js_num1.clone(), complex1.clone(), optable.clone());
    println!(
        "Raised js_num: {}, Raised complex: {}",
        raised_js_num, raised_complex
    );
}
