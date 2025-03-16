use crate::ch2::ch2_4::{apply_generic, attach_tag, contents};
use crate::prelude::*;

pub fn magnitude(z: &List, get: impl Fn(List) -> Option<List> + 'static) -> List {
    println!("magnitude {}", z);
    apply_generic(&"magnitude".to_listv(), z, get).unwrap()
}
pub fn install_rectangular_package(put: impl Fn(List) -> Option<List> + 'static) -> Option<List> {
    let real_part = ClosureWrapper::new(move |x: &List| Some(x.head()));

    let imag_part = ClosureWrapper::new(move |x: &List| Some(x.tail()));

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
    put(list!["real_part", "rectangular", real_part]);
    put(list!["imag_part", "rectangular", imag_part]);
    put(list!["magnitude", "rectangular", magnitude]);
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
    put(list!["magnitude", "polar", magnitude]);
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
pub fn make_complex_from_real_imag(x: List, y: List, get: impl Fn(List) -> Option<List>) -> List {
    get(list!["make_from_real_imag", "complex"])
        .unwrap()
        .try_as_basis_value::<ClosureWrapper>()
        .unwrap()
        .call(&list![x, y])
        .unwrap()
}
