use std::rc::Rc;

use sicp_rs::ch2::ch2_4::{attach_tag, contents, type_tag};
use sicp_rs::ch3::ch3_3::make_table_2d;
use sicp_rs::prelude::*;
// 通用型操作：根据操作符和参数调用对应的函数
fn apply_generic(
    op: &List,
    args: &List,
    get: impl Fn(List) -> Option<List> + 'static,
) -> Option<List> {
    let args = if args.head().is_pair() && args.head().head().is_pair() {
        // 处理可能由于apply_generic导致的嵌套列表
        args.flatmap(|x| x.clone())
    } else {
        args.clone()
    };
    let op_cloned = op.clone();
    println!("apply generic op:{}, args:{}", op_cloned, args);
    let type_tags = args.map(|x| type_tag(x));
    let op = get(list![op.clone(), type_tags]);
    if let Some(op) = op {
        if let Ok(op) = op.try_as_basis_value::<ClosureWrapper>() {
            return op.call(&args.map(|x| contents(x)));
        }
    }
    panic!("No method for these types op:{}, args:{}", op_cloned, args);
}
fn magnitude(z: &List, get: impl Fn(List) -> Option<List> + 'static) -> List {
    println!("magnitude {}", z);
    apply_generic(&"magnitude".to_listv(), &list![z.clone()], get).unwrap()
}
fn install_rectangular_package(put: impl Fn(List) -> Option<List> + 'static) -> Option<List> {
    let real_part = ClosureWrapper::new(move |x: &List| Some(x.head().head()));

    let imag_part = ClosureWrapper::new(move |x: &List| Some(x.head().tail()));

    let (real_cloned, imag_cloned) = (real_part.clone(), imag_part.clone());
    let magnitude = ClosureWrapper::new(move |x: &List| {
        println!("rectangular magnitude {}", x);
        let rp_list = real_cloned.call(x).unwrap();
        let rp = rp_list.try_as_basis_value::<f64>().unwrap();
        let ip_list = imag_cloned.call(x).unwrap();
        let ip = ip_list.try_as_basis_value::<f64>().unwrap();
        Some((rp * rp + ip * ip).sqrt().to_listv())
    });

    let make_from_real_imag = |x: List, y: List| pair![x, y];
    let tag = |x| attach_tag("rectangular", &x);
    // 注意安装操作符时，若action为具体的运算，则key2为list!包裹，list中为所有参与运算的参数的类型
    put(list!["real_part", list!["rectangular"], real_part]);
    put(list!["imag_part", list!["rectangular"], imag_part]);
    put(list!["magnitude", list!["rectangular"], magnitude]);
    // 注意安装操作符时，若action为make，则key2为单值，值为具体的类型名称
    put(list![
        "make_from_real_imag",
        "rectangular",
        ClosureWrapper::new(move |args: &List| {
            let x = args.head();
            let y = args.tail().head();
            Some(tag(make_from_real_imag(x, y)))
        })
    ]);
    Some("done".to_string().to_listv())
}

fn install_polar_package(put: impl Fn(List) -> Option<List> + 'static) -> Option<List> {
    let magnitude = ClosureWrapper::new(move |x: &List| Some(contents(x).head()));
    put(list!["magnitude", list!["polar"], magnitude]);
    Some("done".to_string().to_listv())
}
fn install_complex_packages(
    get: impl Fn(List) -> Option<List> + 'static,
    put: impl Fn(List) -> Option<List> + 'static,
) -> Option<List> {
    let make_from_real_imag = move |x: List, y: List| {
        get(list!["make_from_real_imag", "rectangular"])
            .unwrap()
            .try_as_basis_value::<ClosureWrapper>()
            .unwrap()
            .call(&list![x, y])
            .unwrap()
    };
    let tag = |x| attach_tag("complex", &x);
    let tag_cloned = tag.clone();
    put(list![
        "make_from_real_imag",
        "complex",
        ClosureWrapper::new(move |args: &List| {
            let x = args.head();
            let y = args.tail().head();
            Some(tag_cloned(make_from_real_imag(x.clone(), y.clone())))
        })
    ]);

    Some("done".to_string().to_listv())
}
fn make_complex_from_real_imag(x: List, y: List, get: impl Fn(List) -> Option<List>) -> List {
    get(list!["make_from_real_imag", "complex"])
        .unwrap()
        .try_as_basis_value::<ClosureWrapper>()
        .unwrap()
        .call(&list![x, y])
        .unwrap()
}

fn main() {
    // 创建操作符表
    let optable: Rc<dyn Fn(&str) -> ClosureWrapper> = make_table_2d();
    let op_cloned = optable.clone();
    let get = move |args: List| optable("lookup").call(&args);
    let put = move |args: List| op_cloned("insert").call(&args);
    println!("{:?}", install_rectangular_package(put.clone()));
    println!("{:?}", install_polar_package(put.clone()));
    println!("{:?}", install_complex_packages(get.clone(), put.clone()));
    let a = make_complex_from_real_imag(3.0.to_listv(), 4.0.to_listv(), get.clone());
    println!("{}", a);
    let get_cloned = get.clone();
    let magnitude_wrapper =
        ClosureWrapper::new(move |x: &List| Some(magnitude(x, get_cloned.clone())));
    put(list!["magnitude", list!["complex"], magnitude_wrapper]);
    println!("{}", magnitude(&a, get.clone()))
}
