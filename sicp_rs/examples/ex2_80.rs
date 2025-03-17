use std::rc::Rc;

use sicp_rs::ch2::ch2_4::apply_generic;
use sicp_rs::ch2::ch2_5::imag_part;
use sicp_rs::ch2::ch2_5::install_complex_packages;
use sicp_rs::ch2::ch2_5::install_javascript_number_package;
use sicp_rs::ch2::ch2_5::install_polar_package;
use sicp_rs::ch2::ch2_5::install_rational_package;
use sicp_rs::ch2::ch2_5::install_rectangular_package;
use sicp_rs::ch2::ch2_5::make_complex_from_real_imag;
use sicp_rs::ch2::ch2_5::real_part;
use sicp_rs::ch3::ch3_3::make_table_2d;
use sicp_rs::prelude::*;

fn install_javascript_number_equal_zero(
    put: impl Fn(List) -> Option<List> + 'static,
) -> Option<List> {
    let equal = ClosureWrapper::new(move |args: &List| {
        let x = args.head();
        Some((x == 0.0.to_listv()).to_listv())
    });
    put(list!["is_equal_to_zero", list!["javascript_number"], equal]);
    Some("done".to_string().to_listv())
}
fn install_rational_equal_zero(optable: Rc<dyn Fn(&str) -> ClosureWrapper>) -> Option<List> {
    let op_cloned = optable.clone();
    let get = move |args: List| op_cloned("lookup").call(&args);
    let op_cloned = optable.clone();
    let put = move |args: List| op_cloned("insert").call(&args);
    let numer = get(list!["numer", list!["rational"]]).unwrap();
    let numer = numer
        .try_as_basis_value::<ClosureWrapper>()
        .unwrap()
        .clone();

    let equal = ClosureWrapper::new(move |args: &List| {
        let numer_x = numer.clone().call(&list![args.head()]).unwrap();
        Some((numer_x == 0.to_listv()).to_listv())
    });
    put(list![
        "is_equal_to_zero",
        list!["rational"],
        equal
    ]);
    Some("done".to_string().to_listv())
}

fn install_complex_equal_zero(optable: Rc<dyn Fn(&str) -> ClosureWrapper>) -> Option<List> {
    let op_cloned = optable.clone();
    let get = move |args: List| op_cloned("lookup").call(&args);
    let op_cloned = optable.clone();
    let put = move |args: List| op_cloned("insert").call(&args);

    let get_cloned = get.clone();
    let get_real_imag = move |z: &List| {
        let (get1, get2) = (get_cloned.clone(), get_cloned.clone());
        let (r, i) = (real_part(z, get1), imag_part(z, get2));
        (r, i)
    };
    let equal = ClosureWrapper::new(move |args: &List| {
        let (real_x, imag_x) = get_real_imag(&args.head());
        Some((real_x == 0.0.to_listv() && imag_x == 0.0.to_listv()).to_listv())
    });
    put(list![
        "is_equal_to_zero",
        list!["complex"],
        equal
    ]);
    Some("done".to_string().to_listv())
}

// 需要注意apply_generic的作用: apply_generic剥去标签，并根据标签进行分派。
fn is_equal_to_zero(x: &List, get: impl Fn(List) -> Option<List> + 'static) -> List {
    apply_generic(&"is_equal_to_zero".to_listv(), &list![x.clone()], get).unwrap()
}
fn main() {
    // 创建操作符表
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
    install_complex_equal_zero(optable.clone());
    install_rational_equal_zero(optable.clone());
    install_javascript_number_equal_zero(put.clone());

    // 验证javascript_number的equal规则
    let make_js_number = get(list!["make", "javascript_number"]).unwrap();
    let make_js_number = make_js_number
        .try_as_basis_value::<ClosureWrapper>()
        .unwrap();
    let x = make_js_number.call(&list![0.0]).unwrap();
    println!("x: {}, x==0.0: {}", x, is_equal_to_zero(&x, get.clone()));
    let x = make_js_number.call(&list![1.0]).unwrap();
    println!("x: {}, x==0.0: {}", x, is_equal_to_zero(&x, get.clone()));
    // 验证rational的equal规则
    let make_rational = get(list!["make", "rational"]).unwrap();
    let make_rational = make_rational
        .try_as_basis_value::<ClosureWrapper>()
        .unwrap();
    let x = make_rational.call(&list![0, 2]).unwrap();
    println!("x: {}, x==0.0: {}", x, is_equal_to_zero(&x, get.clone()));
    let x = make_rational.call(&list![1, 3]).unwrap();
    println!("x: {}, x==0.0: {}", x, is_equal_to_zero(&x, get.clone()));
    // 验证complex的equal规则
    let x = make_complex_from_real_imag(0.0.to_listv(), 0.0.to_listv(), get.clone());
    println!("x: {}, x==0.0: {}", x, is_equal_to_zero(&x, get.clone()));
    let x = make_complex_from_real_imag(1.0.to_listv(), 2.0.to_listv(), get.clone());
    println!("x: {}, x==0.0: {}", x, is_equal_to_zero(&x, get.clone()));
}
