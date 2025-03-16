use std::rc::Rc;

use num::Integer;

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
pub fn real_part(z: &List, get: impl Fn(List) -> Option<List> + 'static) -> List {
    apply_generic(&"real_part".to_listv(), &list![z.clone()], get).unwrap()
}
pub fn imag_part(z: &List, get: impl Fn(List) -> Option<List> + 'static) -> List {
    apply_generic(&"imag_part".to_listv(), &list![z.clone()], get).unwrap()
}
pub fn magnitude(z: &List, get: impl Fn(List) -> Option<List> + 'static) -> List {
    apply_generic(&"magnitude".to_listv(), &list![z.clone()], get).unwrap()
}
pub fn angle(z: &List, get: impl Fn(List) -> Option<List> + 'static) -> List {
    apply_generic(&"angle".to_listv(), &list![z.clone()], get).unwrap()
}
pub fn install_javascript_number_package(
    put: impl Fn(List) -> Option<List> + 'static,
) -> Option<List> {
    let tag = |x| attach_tag("javascript_number", &x);
    // put将以操作符和操作数类型为key，将操作函数放入二维表格中。
    // 由于泛型函数为一集函数，而非一个函数，我们无法将一集函数放入二维表格，故此处仅按照f64来实现。
    let get_x_y = |args: &List| {
        let (x, y) = (args.head(), args.tail().head());
        let (x, y) = (
            x.try_as_basis_value::<f64>().unwrap(),
            y.try_as_basis_value::<f64>().unwrap(),
        );
        (*x, *y)
    };
    put(list![
        "add",
        list!["javascript_number", "javascript_number"],
        ClosureWrapper::new(move |args: &List| {
            let (x, y) = get_x_y(args);
            Some(tag((x + y).to_listv()))
        })
    ]);
    put(list![
        "sub",
        list!["javascript_number", "javascript_number"],
        ClosureWrapper::new(move |args: &List| {
            let (x, y) = get_x_y(args);
            Some(tag((x - y).to_listv()))
        })
    ]);
    put(list![
        "mul",
        list!["javascript_number", "javascript_number"],
        ClosureWrapper::new(move |args: &List| {
            let (x, y) = get_x_y(args);
            Some(tag((x * y).to_listv()))
        })
    ]);
    put(list![
        "div",
        list!["javascript_number", "javascript_number"],
        ClosureWrapper::new(move |args: &List| {
            let (x, y) = get_x_y(args);
            Some(tag((x / y).to_listv()))
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
pub fn install_rational_package(put: impl Fn(List) -> Option<List> + 'static) -> Option<List> {
    let numer = ClosureWrapper::new(move |x: &List| Some(x.head().head()));

    let denom = ClosureWrapper::new(move |x: &List| Some(x.head().tail()));
    let make_rat = ClosureWrapper::new(move |args: &List| {
        let (n, d) = (args.head(), args.tail().head());
        let (n, d) = (
            n.try_as_basis_value::<i32>().unwrap(),
            d.try_as_basis_value::<i32>().unwrap(),
        );
        let g = (*n).gcd(d);
        Some(pair!(n / g, d / g))
    });
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

    let (make_rat_cloned, get_numer_and_denom_cloned) =
        (make_rat.clone(), get_numer_and_denom.clone());
    let add_rat = ClosureWrapper::new(move |args| {
        let (numer_x, denom_x, numer_y, denom_y) = get_numer_and_denom_cloned(args);
        make_rat_cloned.call(&list![
            numer_x * denom_y + numer_y * denom_x,
            denom_x * denom_y
        ])
    });

    let (make_rat_cloned, get_numer_and_denom_cloned) =
        (make_rat.clone(), get_numer_and_denom.clone());
    let sub_rat = ClosureWrapper::new(move |args| {
        let (numer_x, denom_x, numer_y, denom_y) = get_numer_and_denom_cloned(args);
        make_rat_cloned.call(&list![
            numer_x * denom_y - numer_y * denom_x,
            denom_x * denom_y
        ])
    });

    let (make_rat_cloned, get_numer_and_denom_cloned) =
        (make_rat.clone(), get_numer_and_denom.clone());
    let mul_rat = ClosureWrapper::new(move |args| {
        let (numer_x, denom_x, numer_y, denom_y) = get_numer_and_denom_cloned(args);
        make_rat_cloned.call(&list![numer_x * numer_y, denom_x * denom_y])
    });

    let (make_rat_cloned, get_numer_and_denom_cloned) =
        (make_rat.clone(), get_numer_and_denom.clone());
    let div_rat = ClosureWrapper::new(move |args| {
        let (numer_x, denom_x, numer_y, denom_y) = get_numer_and_denom_cloned(args);
        make_rat_cloned.call(&list![numer_x * denom_y, denom_x * numer_y])
    });
    let tag = |x| attach_tag("rational", &x);

    put(list!["numer", list!["rational"], numer]);
    put(list!["denom", list!["rational"], denom]);
    let tag_cloned = tag.clone();
    put(list![
        "add",
        list!["rational", "rational"],
        ClosureWrapper::new(move |args| { Some(tag_cloned(add_rat.call(args).unwrap())) })
    ]);
    put(list![
        "sub",
        list!["rational", "rational"],
        ClosureWrapper::new(move |args| { Some(tag_cloned(sub_rat.call(args).unwrap())) })
    ]);
    put(list![
        "mul",
        list!["rational", "rational"],
        ClosureWrapper::new(move |args| { Some(tag_cloned(mul_rat.call(args).unwrap())) })
    ]);
    put(list![
        "div",
        list!["rational", "rational"],
        ClosureWrapper::new(move |args| { Some(tag_cloned(div_rat.call(args).unwrap())) })
    ]);
    put(list![
        "make",
        "rational",
        ClosureWrapper::new(move |args| { Some(tag_cloned(make_rat.call(args).unwrap())) })
    ]);
    Some("done".to_string().to_listv())
}
pub fn make_rational(n: List, d: List, get: impl Fn(List) -> Option<List> + 'static) -> List {
    get(list!["make", "rational"])
        .unwrap()
        .try_as_basis_value::<ClosureWrapper>()
        .unwrap()
        .call(&list![n, d])
        .unwrap()
}
pub fn install_rectangular_package(put: impl Fn(List) -> Option<List> + 'static) -> Option<List> {
    let real_part = ClosureWrapper::new(move |x: &List| Some(x.head().head()));

    let imag_part = ClosureWrapper::new(move |x: &List| Some(x.head().tail()));
    let (real_cloned, imag_cloned) = (real_part.clone(), imag_part.clone());

    let get_real_imag = move |args: &List| {
        let (r, i) = (
            real_cloned.call(args).unwrap(),
            imag_cloned.call(args).unwrap(),
        );
        let (r, i) = (
            r.try_as_basis_value::<f64>().unwrap(),
            i.try_as_basis_value::<f64>().unwrap(),
        );
        (*r, *i)
    };
    let get_real_imag_cloned = get_real_imag.clone();
    let magnitude = ClosureWrapper::new(move |args: &List| {
        let (r, i) = get_real_imag_cloned(args);
        Some((r * r + i * i).sqrt().to_listv())
    });
    let get_real_imag_cloned = get_real_imag.clone();
    let angle = ClosureWrapper::new(move |args: &List| {
        let (r, i) = get_real_imag_cloned(args);
        Some((i.atan2(r)).to_listv())
    });

    let make_from_real_imag = |x: List, y: List| pair![x, y];

    let make_from_mag_ang = move |r: List, a: List| {
        let (r, a) = (
            r.try_as_basis_value::<f64>().unwrap(),
            a.try_as_basis_value::<f64>().unwrap(),
        );
        make_from_real_imag((r * a.cos()).to_listv(), (r * a.sin()).to_listv())
    };
    let tag = |x| attach_tag("rectangular", &x);
    // 注意安装操作符时，若action为具体的运算，则key2为list!包裹，list中为所有参与运算的参数的类型
    put(list!["real_part", list!["rectangular"], real_part]);
    put(list!["imag_part", list!["rectangular"], imag_part]);
    put(list!["magnitude", list!["rectangular"], magnitude]);
    put(list!["angle", list!["rectangular"], angle]);

    // 注意安装操作符时，若action为make，则key2为单值，值为具体的类型名称
    put(list![
        "make_from_real_imag",
        "rectangular",
        ClosureWrapper::new(move |args: &List| {
            let (x, y) = (args.head(), args.tail().head());
            Some(tag(make_from_real_imag(x, y)))
        })
    ]);
    put(list![
        "make_from_mag_ang",
        "rectangular",
        ClosureWrapper::new(move |args: &List| {
            let (r, a) = (args.head(), args.tail().head());
            Some(tag(make_from_mag_ang(r, a)))
        })
    ]);
    Some("done".to_string().to_listv())
}

pub fn install_polar_package(put: impl Fn(List) -> Option<List> + 'static) -> Option<List> {
    let magnitude = ClosureWrapper::new(move |x: &List| Some(contents(x).head().head()));
    let angle = ClosureWrapper::new(move |x: &List| Some(contents(x).head().tail()));
    let (magnitude_cloned, angle_cloned) = (magnitude.clone(), angle.clone());
    let get_magnitude_and_angle = move |args: &List| {
        let (m, a) = (
            magnitude_cloned.call(args).unwrap(),
            angle_cloned.call(args).unwrap(),
        );
        let (m, a) = (
            m.try_as_basis_value::<f64>().unwrap(),
            a.try_as_basis_value::<f64>().unwrap(),
        );
        (*m, *a)
    };
    let get_magnitude_and_angle_cloned = get_magnitude_and_angle.clone();
    let real_part = ClosureWrapper::new(move |args: &List| {
        let (m, a) = get_magnitude_and_angle_cloned(args);
        Some((m * a.cos()).to_listv())
    });
    let get_magnitude_and_angle_cloned = get_magnitude_and_angle.clone();
    let imag_part = ClosureWrapper::new(move |args: &List| {
        let (m, a) = get_magnitude_and_angle_cloned(args);
        Some((m * a.sin()).to_listv())
    });
    let make_from_mag_ang = move |r: List, a: List| pair![r, a];
    let make_from_real_imag = move |r: List, a: List| {
        let (r, a) = (
            r.try_as_basis_value::<f64>().unwrap(),
            a.try_as_basis_value::<f64>().unwrap(),
        );
        pair![(r * r + a * a).sqrt(), (a.atan2(*r))]
    };

    let tag = |x| attach_tag("polar", &x);
    put(list!["magnitude", list!["polar"], magnitude]);
    put(list!["angle", list!["polar"], angle]);
    put(list!["real_part", list!["polar"], real_part]);
    put(list!["imag_part", list!["polar"], imag_part]);
    put(list![
        "make_from_real_imag",
        "polar",
        ClosureWrapper::new(move |args| {
            let (x, y) = (args.head(), args.tail().head());
            Some(tag(make_from_real_imag(x, y)))
        })
    ]);
    put(list![
        "make_from_mag_ang",
        "polar",
        ClosureWrapper::new(move |args| {
            let (x, y) = (args.head(), args.tail().head());
            Some(tag(make_from_mag_ang(x, y)))
        })
    ]);
    Some("done".to_string().to_listv())
}
pub fn install_complex_packages(optable: Rc<dyn Fn(&str) -> ClosureWrapper>) -> Option<List> {
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
        get_closure(get_cloned(list!["make_from_real_imag", "rectangular"]))
            .call(&list![x, y])
            .unwrap()
    };
    let get_cloned = get.clone();
    let make_from_mag_ang = move |r: List, a: List| {
        get_closure(get_cloned(list!["make_from_mag_ang", "polar"]))
            .call(&list![r, a])
            .unwrap()
    };
    let get_cloned = get.clone();
    let get_real_imag = move |z: &List| {
        let (get1, get2) = (get_cloned.clone(), get_cloned.clone());
        let (r, i) = (real_part(z, get1), imag_part(z, get2));
        let (r, i) = (
            r.try_as_basis_value::<f64>().unwrap(),
            i.try_as_basis_value::<f64>().unwrap(),
        );
        (*r, *i)
    };
    let get_real_imag_cloned = get_real_imag.clone();
    let make_from_real_imag_cloned = make_from_real_imag.clone();
    let add_complex = move |z1: &List, z2: &List| {
        let (r1, i1) = get_real_imag_cloned(z1);
        let (r2, i2) = get_real_imag_cloned(z2);
        make_from_real_imag_cloned((r1 + r2).to_listv(), (i1 + i2).to_listv())
    };
    let get_real_imag_cloned = get_real_imag.clone();
    let make_from_real_imag_cloned = make_from_real_imag.clone();
    let sub_complex = move |z1: &List, z2: &List| {
        let (r1, i1) = get_real_imag_cloned(z1);
        let (r2, i2) = get_real_imag_cloned(z2);

        make_from_real_imag_cloned((r1 - r2).to_listv(), (i1 - i2).to_listv())
    };
    let get_cloned = get.clone();
    let get_magnitude_and_angle = move |z: &List| {
        let (get1, get2) = (get_cloned.clone(), get_cloned.clone());
        let (r, a) = (magnitude(z, get1), angle(z, get2));
        let (r, a) = (
            r.try_as_basis_value::<f64>().unwrap(),
            a.try_as_basis_value::<f64>().unwrap(),
        );
        (*r, *a)
    };
    let get_mantitude_and_angle_cloned = get_magnitude_and_angle.clone();
    let make_from_mag_ang_cloned = make_from_mag_ang.clone();

    let mul_complex = move |z1: &List, z2: &List| {
        let (r1, i1) = get_mantitude_and_angle_cloned(z1);
        let (r2, i2) = get_mantitude_and_angle_cloned(z2);
        make_from_mag_ang_cloned((r1 * r2).to_listv(), (i1 + i2).to_listv())
    };
    let get_mantitude_and_angle_cloned = get_magnitude_and_angle.clone();
    let make_from_mag_ang_cloned = make_from_mag_ang.clone();

    let div_complex = move |z1: &List, z2: &List| {
        let (r1, i1) = get_mantitude_and_angle_cloned(z1);
        let (r2, i2) = get_mantitude_and_angle_cloned(z2);
        make_from_mag_ang_cloned((r1 / r2).to_listv(), (i1 - i2).to_listv())
    };
    let tag = |x| attach_tag("complex", &x);
    put(list![
        "add",
        list!["complex", "complex"],
        ClosureWrapper::new(move |args: &List| {
            let (z1, z2) = (args.head(), args.tail().head());
            Some(tag(add_complex(&z1, &z2)))
        })
    ]);
    put(list![
        "sub",
        list!["complex", "complex"],
        ClosureWrapper::new(move |args: &List| {
            let (z1, z2) = (args.head(), args.tail().head());
            Some(tag(sub_complex(&z1, &z2)))
        })
    ]);
    put(list![
        "mul",
        list!["complex", "complex"],
        ClosureWrapper::new(move |args: &List| {
            let (z1, z2) = (args.head(), args.tail().head());
            Some(tag(mul_complex(&z1, &z2)))
        })
    ]);
    put(list![
        "div",
        list!["complex", "complex"],
        ClosureWrapper::new(move |args: &List| {
            let (z1, z2) = (args.head(), args.tail().head());
            Some(tag(div_complex(&z1, &z2)))
        })
    ]);

    put(list![
        "make_from_real_imag",
        "complex",
        ClosureWrapper::new(move |args: &List| {
            let x = args.head();
            let y = args.tail().head();
            Some(tag(make_from_real_imag(x.clone(), y.clone())))
        })
    ]);
    put(list![
        "make_from_mag_ang",
        "complex",
        ClosureWrapper::new(move |args: &List| {
            let (x, y) = (args.head(), args.tail().head());
            Some(tag(make_from_mag_ang(x.clone(), y.clone())))
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
pub fn make_complex_from_mag_ang(
    r: List,
    a: List,
    get: impl Fn(List) -> Option<List> + 'static,
) -> List {
    get(list!["make_from_mag_ang", "complex"])
        .unwrap()
        .try_as_basis_value::<ClosureWrapper>()
        .unwrap()
        .call(&list![r, a])
        .unwrap()
}
