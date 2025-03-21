use std::rc::Rc;

use num::Integer;

use crate::ch2::ch2_4::{apply_generic, attach_tag, contents};
use crate::prelude::*;

use super::ch2_4::type_tag;
const COMPLEX_ERROR_MESSAGE: &str = "complex only supports f64, please construct complex with f64";
const JAVASCRIPT_NUMBER_ERROR_MESSAGE: &str =
    "javascript number only supports f64, please construct javascript number with f64";
const JAVASCRIPT_INTEGER_ERROR_MESSAGE: &str =
    "javascript integer only supports i32, please construct javascript integer with i32";
const RATIONAL_ERROR_MESSAGE: &str =
    "rational only supports f64, please construct rational with f64";

pub fn add(
    x: &List,
    y: &List,
    get: impl Fn(List) -> Option<List> + 'static,
    coercion: &List,
) -> List {
    if coercion.is_empty() {
        apply_generic(&"add".to_listv(), &list![x.clone(), y.clone()], get).unwrap()
    } else {
        apply_generic(
            &pair![list!["coercion", coercion.clone()], "add"],
            &list![x.clone(), y.clone()],
            get,
        )
        .unwrap()
    }
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
pub fn numer(z: &List, get: impl Fn(List) -> Option<List> + 'static) -> List {
    apply_generic(&"numer".to_listv(), &list![z.clone()], get).unwrap()
}
pub fn denom(z: &List, get: impl Fn(List) -> Option<List> + 'static) -> List {
    apply_generic(&"denom".to_listv(), &list![z.clone()], get).unwrap()
}
pub fn magnitude(z: &List, get: impl Fn(List) -> Option<List> + 'static) -> List {
    apply_generic(&"magnitude".to_listv(), &list![z.clone()], get).unwrap()
}
pub fn angle(z: &List, get: impl Fn(List) -> Option<List> + 'static) -> List {
    apply_generic(&"angle".to_listv(), &list![z.clone()], get).unwrap()
}
pub fn is_equal(x: &List, y: &List, get: impl Fn(List) -> Option<List> + 'static) -> List {
    apply_generic(&"equal".to_listv(), &list![x.clone(), y.clone()], get).unwrap()
}
pub fn is_equal_to_zero(x: &List, get: impl Fn(List) -> Option<List> + 'static) -> List {
    apply_generic(&"is_equal_to_zero".to_listv(), &list![x.clone()], get).unwrap()
}
pub fn raise(x: &List, get: impl Fn(List) -> Option<List> + 'static) -> List {
    if !is_arithmetic_type(x) || type_tag(x) == "complex".to_listv() {
        x.clone()
    } else {
        apply_generic(&"raise".to_listv(), &list![x.clone()], get).unwrap()
    }
}
pub fn project(x: &List, get: impl Fn(List) -> Option<List> + 'static) -> List {
    if !is_arithmetic_type(x) || type_tag(x) == "integer".to_listv() {
        x.clone()
    } else {
        apply_generic(&"project".to_listv(), &list![x.clone()], get).unwrap()
    }
}

pub fn install_arithmetic_raise_package(
    optable: Rc<dyn Fn(&str) -> ClosureWrapper>,
) -> Option<List> {
    let op_cloned = optable.clone();
    let get = move |args: List| optable("lookup").call(&args);
    let put = move |args: List| op_cloned("insert").call(&args);
    let get_cloned = get.clone();
    put(list![
        "raise",
        list!["integer"],
        ClosureWrapper::new(move |args| {
            Some(make_rational(args.head(), 1.to_listv(), get_cloned.clone()))
        })
    ]);
    let get_cloned = get.clone();
    put(list![
        "raise",
        list!["rational"],
        ClosureWrapper::new(move |args| {
            let (numer_x, denom_x) = (args.head().head(), args.head().tail());
            let (numer_x, denom_x) = (
                numer_x
                    .try_as_basis_value::<i32>()
                    .expect(RATIONAL_ERROR_MESSAGE),
                denom_x
                    .try_as_basis_value::<i32>()
                    .expect(RATIONAL_ERROR_MESSAGE),
            );
            Some(make_javascript_number(
                ((*numer_x as f64) / (*denom_x as f64)).to_listv(),
                get_cloned.clone(),
            ))
        })
    ]);
    let get_cloned = get.clone();
    put(list![
        "raise",
        list!["javascript_number"],
        ClosureWrapper::new(move |args| {
            let i = args.head();

            Some(make_complex_from_real_imag(
                i,
                0.0.to_listv(),
                get_cloned.clone(),
            ))
        })
    ]);
    Some("done".to_string().to_listv())
}
pub fn install_arithmetic_project_package(optable: Rc<dyn Fn(&str) -> ClosureWrapper>) -> Option<List> {
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

pub fn install_javascript_integer_package(
    put: impl Fn(List) -> Option<List> + 'static,
) -> Option<List> {
    let tag = |x| attach_tag("integer", &x);
    // put将以操作符和操作数类型为key，将操作函数放入二维表格中。
    // 由于泛型函数为一集函数，而非一个函数，我们无法将一集函数放入二维表格，故此处仅按照i32来实现。
    let get_x_y = |args: &List| {
        let (x, y) = (args.head(), args.tail().head());
        let (x, y) = (
            x.try_as_basis_value::<i32>()
                .expect(JAVASCRIPT_INTEGER_ERROR_MESSAGE),
            y.try_as_basis_value::<i32>()
                .expect(JAVASCRIPT_INTEGER_ERROR_MESSAGE),
        );
        (*x, *y)
    };
    put(list![
        "add",
        list!["integer", "integer"],
        ClosureWrapper::new(move |args: &List| {
            let (x, y) = get_x_y(args);
            Some(tag((x + y).to_listv()))
        })
    ]);
    put(list![
        "sub",
        list!["integer", "integer"],
        ClosureWrapper::new(move |args: &List| {
            let (x, y) = get_x_y(args);
            Some(tag((x - y).to_listv()))
        })
    ]);
    put(list![
        "mul",
        list!["integer", "integer"],
        ClosureWrapper::new(move |args: &List| {
            let (x, y) = get_x_y(args);
            Some(tag((x * y).to_listv()))
        })
    ]);
    put(list![
        "div",
        list!["integer", "integer"],
        ClosureWrapper::new(move |args: &List| {
            let (x, y) = get_x_y(args);
            Some(tag((x / y).to_listv()))
        })
    ]);
    put(list![
        "equal",
        list!["integer", "integer"],
        ClosureWrapper::new(move |args: &List| {
            let (x, y) = (args.head(), args.tail().head());
            Some((x == y).to_listv())
        })
    ]);

    put(list![
        "is_equal_to_zero",
        list!["integer"],
        ClosureWrapper::new(move |args: &List| { Some((args.head() == 0.to_listv()).to_listv()) })
    ]);
    put(list![
        "make",
        "integer",
        ClosureWrapper::new(move |x: &List| { Some(tag(x.head())) })
    ]);

    Some("done".to_string().to_listv())
}
pub fn make_javascript_integer(x: List, get: impl Fn(List) -> Option<List> + 'static) -> List {
    get(list!["make", "integer"])
        .unwrap()
        .try_as_basis_value::<ClosureWrapper>()
        .unwrap()
        .call(&list![x])
        .unwrap()
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
            x.try_as_basis_value::<f64>()
                .expect(JAVASCRIPT_NUMBER_ERROR_MESSAGE),
            y.try_as_basis_value::<f64>()
                .expect(JAVASCRIPT_NUMBER_ERROR_MESSAGE),
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
        "equal",
        list!["javascript_number", "javascript_number"],
        ClosureWrapper::new(move |args: &List| {
            let (x, y) = (args.head(), args.tail().head());
            Some((x == y).to_listv())
        })
    ]);

    put(list![
        "is_equal_to_zero",
        list!["javascript_number"],
        ClosureWrapper::new(move |args: &List| {
            Some((args.head() == 0.0.to_listv()).to_listv())
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
            n.try_as_basis_value::<i32>().expect(RATIONAL_ERROR_MESSAGE),
            d.try_as_basis_value::<i32>().expect(RATIONAL_ERROR_MESSAGE),
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
            numer_x
                .try_as_basis_value::<i32>()
                .expect(RATIONAL_ERROR_MESSAGE),
            denom_x
                .try_as_basis_value::<i32>()
                .expect(RATIONAL_ERROR_MESSAGE),
            numer_y
                .try_as_basis_value::<i32>()
                .expect(RATIONAL_ERROR_MESSAGE),
            denom_y
                .try_as_basis_value::<i32>()
                .expect(RATIONAL_ERROR_MESSAGE),
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
    let numer_cloned = numer.clone();
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
        "equal",
        list!["rational", "rational"],
        ClosureWrapper::new(move |args: &List| {
            let (numer_x, denom_x, numer_y, denom_y) = get_numer_and_denom(args);
            Some((numer_x == numer_y && denom_x == denom_y).to_listv())
        })
    ]);
    put(list![
        "is_equal_to_zero",
        list!["rational"],
        ClosureWrapper::new(move |args: &List| {
            let numer_x = numer_cloned.clone().call(&list![args.head()]).unwrap();
            Some((numer_x == 0.to_listv()).to_listv())
        })
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
            r.try_as_basis_value::<f64>().expect(COMPLEX_ERROR_MESSAGE),
            i.try_as_basis_value::<f64>().expect(COMPLEX_ERROR_MESSAGE),
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
            r.try_as_basis_value::<f64>().expect(COMPLEX_ERROR_MESSAGE),
            a.try_as_basis_value::<f64>().expect(COMPLEX_ERROR_MESSAGE),
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
            m.try_as_basis_value::<f64>().expect(COMPLEX_ERROR_MESSAGE),
            a.try_as_basis_value::<f64>().expect(COMPLEX_ERROR_MESSAGE),
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
            r.try_as_basis_value::<f64>().expect(COMPLEX_ERROR_MESSAGE),
            a.try_as_basis_value::<f64>().expect(COMPLEX_ERROR_MESSAGE),
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
            r.try_as_basis_value::<f64>().expect(COMPLEX_ERROR_MESSAGE),
            i.try_as_basis_value::<f64>().expect(COMPLEX_ERROR_MESSAGE),
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
            r.try_as_basis_value::<f64>().expect(COMPLEX_ERROR_MESSAGE),
            a.try_as_basis_value::<f64>().expect(COMPLEX_ERROR_MESSAGE),
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
    let get_real_imag_cloned = get_real_imag.clone();
    let equal = ClosureWrapper::new(move |args: &List| {
        let (real_x, imag_x) = get_real_imag_cloned(&args.head());
        let (real_y, imag_y) = get_real_imag_cloned(&args.tail().head());
        Some((real_x == real_y && imag_x == imag_y).to_listv())
    });
    let get_cloned = get.clone();
    let is_equal_to_zero = ClosureWrapper::new(move |args: &List| {
        let z = args.head();
        let (real_x, imag_x) = (
            real_part(&z, get_cloned.clone()),
            imag_part(&z, get_cloned.clone()),
        );
        Some((real_x == 0.0.to_listv() && imag_x == 0.0.to_listv()).to_listv())
    });
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
    put(list!["equal", list!["complex", "complex"], equal]);
    put(list![
        "is_equal_to_zero",
        list!["complex"],
        is_equal_to_zero
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
pub fn install_arithmetic_package(optable: Rc<dyn Fn(&str) -> ClosureWrapper>) -> Option<List> {
    let op_cloned = optable.clone();
    let put = move |args: List| op_cloned("insert").call(&args);
    install_complex_packages(optable.clone());
    install_rectangular_package(put.clone());
    install_polar_package(put.clone());
    install_rational_package(put.clone());
    install_javascript_number_package(put.clone());
    install_javascript_integer_package(put.clone());
    install_arithmetic_raise_package(optable.clone());
    install_arithmetic_project_package(optable.clone());
    Some("done".to_string().to_listv())
}
// coercion support
pub fn put_coercion(
    type1: &List,
    type2: &List,
    proc: ClosureWrapper,
    coercion_list: &List,
) -> List {
    if get_coercion(type1, type2, coercion_list).is_none() {
        pair![
            list![type1.clone(), type2.clone(), proc],
            coercion_list.clone()
        ]
    } else {
        coercion_list.clone()
    }
}
pub fn get_coercion(type1: &List, type2: &List, coercion_list: &List) -> Option<List> {
    fn get_type1(list_item: &List) -> List {
        list_item.head()
    }
    fn get_type2(list_item: &List) -> List {
        list_item.tail().head()
    }
    fn get_proc(list_item: &List) -> List {
        list_item.tail().tail().head()
    }
    fn get_coercion_iter(type1: &List, type2: &List, items: &List) -> Option<List> {
        if items.is_empty() {
            None
        } else {
            let top = items.head();

            if get_type1(&top) == *type1 && get_type2(&top) == *type2 {
                Some(get_proc(&top))
            } else {
                get_coercion_iter(type1, type2, &items.tail())
            }
        }
    }
    get_coercion_iter(type1, type2, coercion_list)
}

const ARITHMETIC_TYPES: [&str; 4] = ["integer", "rational", "javascript_number", "complex"];

pub fn find_arithmetic_type_index(type_tag: &str) -> i32 {
    for (i, t) in ARITHMETIC_TYPES.iter().enumerate() {
        if type_tag == *t {
            return i as i32;
        }
    }
    -1
}
pub fn is_arithmetic_type(x: &List) -> bool {
    find_arithmetic_type_index(&type_tag(x).to_string()) >= 0
}
pub fn arithmetic_type_raise(
    a1: List,
    a2: List,
    optable: Rc<dyn Fn(&str) -> ClosureWrapper>,
) -> (List, List) {
    let a1_type_tag = type_tag(&a1);
    let a2_type_tag = type_tag(&a2);
    let a1_index = find_arithmetic_type_index(&a1_type_tag.to_string());
    let a2_index = find_arithmetic_type_index(&a2_type_tag.to_string());
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

pub fn drop(x: &List, optable: Rc<dyn Fn(&str) -> ClosureWrapper>) -> List {
    let op_cloned = optable.clone();
    let get = move |args: List| optable("lookup").call(&args);
    let new_x = project(x, get.clone());
    if raise(&new_x, get.clone()) == *x {
        drop(&new_x, op_cloned)
    } else {
        x.clone()
    }
}
pub fn apply_generic_drop_wrapper(
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