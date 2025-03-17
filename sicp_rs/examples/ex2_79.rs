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

fn install_javascript_number_equal(put: impl Fn(List) -> Option<List> + 'static) -> Option<List> {
    let equal = ClosureWrapper::new(move |args: &List| {
        let x = args.head();
        let y = args.tail().head();
        Some((x == y).to_listv())
    });
    put(list![
        "equal",
        list!["javascript_number", "javascript_number"],
        equal
    ]);
    Some("done".to_string().to_listv())
}
fn install_rational_equal(optable: Rc<dyn Fn(&str) -> ClosureWrapper>) -> Option<List> {
    let op_cloned = optable.clone();
    let get = move |args: List| op_cloned("lookup").call(&args);
    let op_cloned = optable.clone();
    let put = move |args: List| op_cloned("insert").call(&args);
    let numer = get(list!["numer", list!["rational"]]).unwrap();
    let numer = numer.try_as_basis_value::<ClosureWrapper>().unwrap();
    let denom = get(list!["denom", list!["rational"]]).unwrap();
    let denom = denom.try_as_basis_value::<ClosureWrapper>().unwrap();
    let (numer_cloned, denom_cloned) = (numer.clone(), denom.clone());

    let get_numer_and_denom = move |args: &List| {
        let (x, y) = (args.head(), args.tail().head());
        let (numer_x, denom_x, numer_y, denom_y) = (
            numer_cloned.clone().call(&list![x.clone()]).unwrap(),
            denom_cloned.clone().call(&list![x]).unwrap(),
            numer_cloned.clone().call(&list![y.clone()]).unwrap(),
            denom_cloned.clone().call(&list![y]).unwrap(),
        );
        let (numer_x, denom_x, numer_y, denom_y) = (
            numer_x.try_as_basis_value::<i32>().unwrap(),
            denom_x.try_as_basis_value::<i32>().unwrap(),
            numer_y.try_as_basis_value::<i32>().unwrap(),
            denom_y.try_as_basis_value::<i32>().unwrap(),
        );
        (*numer_x, *denom_x, *numer_y, *denom_y)
    };
    let equal = ClosureWrapper::new(move |args: &List| {
        let (numer_x, denom_x, numer_y, denom_y) = get_numer_and_denom(args);
        Some((numer_x == numer_y && denom_x == denom_y).to_listv())
    });
    put(list!["equal", list!["rational", "rational"], equal]);
    Some("done".to_string().to_listv())
}

fn install_complex_equal(optable: Rc<dyn Fn(&str) -> ClosureWrapper>) -> Option<List> {
    let op_cloned = optable.clone();
    let get = move |args: List| op_cloned("lookup").call(&args);
    let op_cloned = optable.clone();
    let put = move |args: List| op_cloned("insert").call(&args);

    let get_real_and_imag = move |args: &List| {
        let (x, y) = (args.head(), args.tail().head());
        let (real_x, imag_x, real_y, imag_y) = (
            real_part(&x, get.clone()),
            imag_part(&x, get.clone()),
            real_part(&y, get.clone()),
            imag_part(&y, get.clone()),
        );
        let (real_x, imag_x, real_y, imag_y) = (
            real_x.try_as_basis_value::<f64>().unwrap(),
            imag_x.try_as_basis_value::<f64>().unwrap(),
            real_y.try_as_basis_value::<f64>().unwrap(),
            imag_y.try_as_basis_value::<f64>().unwrap(),
        );
        (*real_x, *imag_x, *real_y, *imag_y)
    };
    let equal = ClosureWrapper::new(move |args: &List| {
        let (real_x, imag_x, real_y, imag_y) = get_real_and_imag(args);
        Some((real_x == real_y && imag_x == imag_y).to_listv())
    });
    put(list!["equal", list!["complex", "complex"], equal]);
    Some("done".to_string().to_listv())
}

// 需要注意apply_generic的作用: apply_generic剥去标签，并根据标签进行分派。
fn is_equal(x: &List, y: &List, get: impl Fn(List) -> Option<List> + 'static) -> List {
    apply_generic(&"equal".to_listv(), &list![x.clone(), y.clone()], get).unwrap()
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
    install_complex_equal(optable.clone());
    install_rational_equal(optable.clone());
    install_javascript_number_equal(put.clone());

    // 验证javascript_number的equal规则
    let make_js_number = get(list!["make", "javascript_number"]).unwrap();
    let make_js_number = make_js_number
        .try_as_basis_value::<ClosureWrapper>()
        .unwrap();
    let x = make_js_number.call(&list![1.0]).unwrap();
    let y = make_js_number.call(&list![1.0]).unwrap();
    println!(
        "x: {}, y: {}, x==y: {}",
        x,
        y,
        is_equal(&x, &y, get.clone())
    );
    let x = make_js_number.call(&list![1.0]).unwrap();
    let y = make_js_number.call(&list![2.0]).unwrap();
    println!(
        "x: {}, y: {}, x==y: {}",
        x,
        y,
        is_equal(&x, &y, get.clone())
    );
    // 验证rational的equal规则
    let make_rational = get(list!["make", "rational"]).unwrap();
    let make_rational = make_rational
        .try_as_basis_value::<ClosureWrapper>()
        .unwrap();

    let x = make_rational.call(&list![1, 2]).unwrap();
    let y = make_rational.call(&list![2, 4]).unwrap();
    println!(
        "x: {}, y: {}, x==y: {}",
        x,
        y,
        is_equal(&x, &y, get.clone())
    );
    let x = make_rational.call(&list![1, 3]).unwrap();
    let y = make_rational.call(&list![2, 4]).unwrap();
    println!(
        "x: {}, y: {}, x==y: {}",
        x,
        y,
        is_equal(&x, &y, get.clone())
    );
    // 验证complex的equal规则
    let x = make_complex_from_real_imag(1.0.to_listv(), 2.0.to_listv(), get.clone());
    let y = make_complex_from_real_imag(1.0.to_listv(), 2.0.to_listv(), get.clone());
    println!(
        "x: {}, y: {}, x==y: {}",
        x,
        y,
        is_equal(&x, &y, get.clone())
    );

    let x = make_complex_from_real_imag(1.0.to_listv(), 2.0.to_listv(), get.clone());
    let y = make_complex_from_real_imag(1.0.to_listv(), 3.0.to_listv(), get.clone());
    println!(
        "x: {}, y: {}, x==y: {}",
        x,
        y,
        is_equal(&x, &y, get.clone())
    )
}
