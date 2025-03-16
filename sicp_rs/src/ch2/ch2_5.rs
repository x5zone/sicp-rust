use crate::ch2::ch2_4::{apply_generic, attach_tag, contents};
use crate::prelude::*;

pub fn add(x: &List, y: &List, get: impl Fn(List) -> Option<List> + 'static) -> List {
    apply_generic(&"add".to_listv(), &list![x.clone(), y.clone()], get).unwrap()
}
pub fn sub(x: &List, y: &List, get: impl Fn(List) -> Option<List> + 'static) -> List {
    apply_generic(&"sub".to_listv(), &list![x.clone(), y.clone()], get).unwrap()
}
pub fn mul(x: &List, y: &List, get: impl Fn(List) -> Option<List> + 'static) -> List {
    apply_generic(&"mul".to_listv(), &list![x.clone(), y.clone()], get).unwrap()
}
pub fn div(x: &List, y: &List, get: impl Fn(List) -> Option<List> + 'static) -> List {
    apply_generic(&"div".to_listv(), &list![x.clone(), y.clone()], get).unwrap()
}
pub fn magnitude(z: &List, get: impl Fn(List) -> Option<List> + 'static) -> List {
    apply_generic(&"magnitude".to_listv(), &list![z.clone()], get).unwrap()
}
pub fn install_javascript_number_package(
    put: impl Fn(List) -> Option<List> + 'static,
) -> Option<List> {
    let tag = |x| attach_tag("javascript_number", &x);
    // put将以操作符和操作数类型为key，将操作函数放入二维表格中。
    // 由于泛型函数为一集函数，而非一个函数，我们无法将一集函数放入二维表格，故此处仅按照f64来实现。
    put(list![
        "add",
        list!["javascript_number", "javascript_number"],
        ClosureWrapper::new(move |args: &List| {
            let x = args.head();
            let y = args.tail().head();
            Some(tag((x.try_as_basis_value::<f64>().unwrap()
                + y.try_as_basis_value::<f64>().unwrap())
            .to_listv()))
        })
    ]);
    put(list![
        "sub",
        list!["javascript_number", "javascript_number"],
        ClosureWrapper::new(move |args: &List| {
            let x = args.head();
            let y = args.tail().head();
            Some(tag((x.try_as_basis_value::<f64>().unwrap()
                - y.try_as_basis_value::<f64>().unwrap())
            .to_listv()))
        })
    ]);
    put(list![
        "mul",
        list!["javascript_number", "javascript_number"],
        ClosureWrapper::new(move |args: &List| {
            let x = args.head();
            let y = args.tail().head();
            Some(tag((x.try_as_basis_value::<f64>().unwrap()
                * y.try_as_basis_value::<f64>().unwrap())
            .to_listv()))
        })
    ]);
    put(list![
        "div",
        list!["javascript_number", "javascript_number"],
        ClosureWrapper::new(move |args: &List| {
            let x = args.head();
            let y = args.tail().head();
            Some(tag((x.try_as_basis_value::<f64>().unwrap()
                / y.try_as_basis_value::<f64>().unwrap())
            .to_listv()))
        })
    ]);
    put(list![
        "make",
        "javascript_number",
        ClosureWrapper::new(move |x: &List| { Some(tag(x.head())) })
    ]);

    Some("done".to_string().to_listv())
}
pub fn make_javascript_number(x: List, get: impl Fn(List) -> Option<List> + 'static) -> List {
    get(list!["make", "javascript_number"])
        .unwrap()
        .try_as_basis_value::<ClosureWrapper>()
        .unwrap()
        .call(&list![x])
        .unwrap()
}
pub fn install_rectangular_package(put: impl Fn(List) -> Option<List> + 'static) -> Option<List> {
    let real_part = ClosureWrapper::new(move |x: &List| Some(x.head()));

    let imag_part = ClosureWrapper::new(move |x: &List| Some(x.tail()));

    let (real_cloned, imag_cloned) = (real_part.clone(), imag_part.clone());
    let magnitude = ClosureWrapper::new(move |x: &List| {
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

pub fn install_polar_package(put: impl Fn(List) -> Option<List> + 'static) -> Option<List> {
    let magnitude = ClosureWrapper::new(move |x: &List| Some(contents(x).head()));
    put(list!["magnitude", list!["polar"], magnitude]);
    Some("done".to_string().to_listv())
}
pub fn install_complex_packages(
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
pub fn make_complex_from_real_imag(
    x: List,
    y: List,
    get: impl Fn(List) -> Option<List> + 'static,
) -> List {
    get(list!["make_from_real_imag", "complex"])
        .unwrap()
        .try_as_basis_value::<ClosureWrapper>()
        .unwrap()
        .call(&list![x, y])
        .unwrap()
}
