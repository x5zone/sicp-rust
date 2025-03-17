use sicp_rs::ch2::ch2_4::apply_generic;
use sicp_rs::ch2::ch2_5::add;
use sicp_rs::ch2::ch2_5::install_complex_packages;
use sicp_rs::ch2::ch2_5::install_javascript_number_package;
use sicp_rs::ch2::ch2_5::install_polar_package;
use sicp_rs::ch2::ch2_5::install_rational_package;
use sicp_rs::ch2::ch2_5::install_rectangular_package;
use sicp_rs::ch2::ch2_5::make_complex_from_real_imag;
use sicp_rs::ch2::ch2_5::make_javascript_number;
use sicp_rs::ch2::ch2_5::put_coercion;
use sicp_rs::ch3::ch3_3::make_table_2d;
use sicp_rs::prelude::*;

fn exp(x: &List, y: &List, get: impl Fn(List) -> Option<List> + 'static, coercion: &List) -> List {
    if coercion.is_empty() {
        apply_generic(&"exp".to_listv(), &list![x.clone(), y.clone()], get).unwrap()
    } else {
        apply_generic(
            &pair![list!["coercion", coercion.clone()], "exp"],
            &list![x.clone(), y.clone()],
            get,
        )
        .unwrap()
    }
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

    let get_cloned = get.clone();
    let javascript_number_to_complex = ClosureWrapper::new(move |args: &List| {
        let args = args.head();
       
        let x = *(args.try_as_basis_value::<f64>().unwrap());
        Some(make_complex_from_real_imag(
            x.to_listv(),
            0.0.to_listv(),
            get_cloned.clone(),
        ))
    });
    let coercion = put_coercion(
        &"javascript_number".to_listv(),
        &"complex".to_listv(),
        javascript_number_to_complex,
        &List::Nil,
    );
   
    let c = make_complex_from_real_imag(4.0.to_listv(), 3.0.to_listv(), get.clone());
    let n = make_javascript_number(7.0.to_listv(), get.clone());
    let get_cloned = get.clone();
    println!(
        "{}, {}, add {}",
        c,
        n,
        add(&c, &n, get_cloned.clone(), &coercion)
    );
    let complex_to_complex = ClosureWrapper::new(move |args: &List| {
        let args = args.head();
        
        Some(args)
    });
    let coercion = put_coercion(
        &"complex".to_listv(),
        &"complex".to_listv(),
        complex_to_complex,
        &coercion,
    );
    put(list![
        "exp",
        list!["javascript_number", "javascript_number"],
        ClosureWrapper::new(move |args: &List| {
            let (base, exp) = (args.head(), args.tail().head());
            let base = base.try_as_basis_value::<f64>().unwrap();
            let exp = exp.try_as_basis_value::<f64>().unwrap();
            Some(make_javascript_number(
                (base.powf(*exp)).to_listv(),
                get_cloned.clone(),
            ))
        })
    ]);
    println!(
        "{}",
        exp(&10.0.to_listv(), &2.0.to_listv(), get.clone(), &coercion)
    );

    //println!("{}", exp(&c, &c, get.clone(), &coercion));
}
