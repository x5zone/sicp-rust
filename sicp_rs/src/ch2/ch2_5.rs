use std::rc::Rc;
use std::{fmt, i32};

use num::{Integer, Num};

use crate::ch3::ch3_3::make_table_2d;
use crate::prelude::*;

pub fn attach_tag(tag: &str, contents: &List) -> List {
    //Only Support f64&i32
    if contents.is_value()
        && (contents.try_as_basis_value::<f64>().is_ok()
            || contents.try_as_basis_value::<i32>().is_ok())
    {
        return contents.clone();
    };
    pair!(tag.to_string(), contents.clone())
}

pub fn type_tag(datum: &List) -> List {
    // Only Support f64&i32
    if datum.is_value() && datum.try_as_basis_value::<f64>().is_ok() {
        "float".to_listv()
    } else if datum.is_value() && datum.try_as_basis_value::<i32>().is_ok() {
        "integer".to_listv()
    } else if datum.is_pair() {
        datum.head()
    } else {
        panic!("bad tagged datum -- TYPE-TAG")
    }
}

pub fn contents(datum: &List) -> List {
    // Only Support f64&i32
    if datum.is_value()
        && (datum.try_as_basis_value::<f64>().is_ok() || datum.try_as_basis_value::<i32>().is_ok())
    {
        datum.clone()
    } else if datum.is_pair() {
        datum.tail()
    } else {
        panic!("bad tagged datum -- CONTENTS")
    }
}
pub fn apply_generic(op: &List, args: &List, arith: &ArithmeticContext) -> Option<List> {
    let args = if args.head().is_pair() && args.head().head().is_pair() {
        // 处理可能由于apply_generic导致的嵌套列表
        args.flatmap(|x| x.clone())
    } else {
        args.clone()
    };

    let type_tags = args.map(|x| type_tag(x));
    let func = arith.get(list![op.clone(), type_tags.clone()]);
    if let Some(func) = func {
        // 找到对应函数签名，直接调用
        func.call(&args.map(|x| contents(x)))
    } else {
        if args.length() != 2 {
            panic!(
                "apply_generic expects 2 args, got {} len {} with op {}, may not found method",
                args,
                args.length(),
                op
            );
        }

        let type1 = type_tags.head();
        let type2 = type_tags.tail().head();
        assert_ne!(type1, type2, "no method found for op:{}, args:{}", op, args);
        let a1 = args.head();
        let a2 = args.tail().head();

        //可提升的算术类型
        if find_arithmetic_type_index(&type1.to_string()) != -1
            && find_arithmetic_type_index(&type2.to_string()) != -1
        {
            let (a1, a2) = unify_arithmetic_types(a1, a2, arith);
            return apply_generic(op, &list![a1, a2], arith);
        }

        // 类型强制
        let try_coerce_and_apply = |t1: &List, t2: &List, a1: &List, a2: &List, direction: i32| {
            if let Some(t1_to_t2) = arith.get_coercion(t1, t2) {
                let coerce = |x: &List| {
                    t1_to_t2
                        .call(&list![x.clone()])
                        .expect(&format!("{} to {} coercion failed", t1, t2))
                };
                let (a1, a2) = match direction {
                    1 => (coerce(a1), a2.clone()),
                    2 => (a1.clone(), coerce(a2)),
                    _ => unreachable!("coerce direction only support 1:t1_to_t2 and 2:t2_to_t1"),
                };
                apply_generic(op, &list![a1, a2], arith)
            } else {
                None
            }
        };
        if let Some(result) = try_coerce_and_apply(&type1, &type2, &a1, &a2, 1)
            .or_else(|| try_coerce_and_apply(&type2, &type1, &a1, &a2, 2))
        {
            Some(result)
        } else {
            panic!("No method for these types op:{}, args:{}", op, args);
        }
    }
}

pub fn install_arithmetic_package(arith: &ArithmeticContext) -> Option<List> {
    install_integer_package(&arith);
    install_float_package(&arith);
    install_rational_package(&arith);
    install_polar_package(&arith);
    install_rectangular_package(&arith);
    install_complex_package(&arith);
    Some("done".to_string().to_listv())
}

const ARITHMETIC_TYPES: [&str; 4] = ["integer", "rational", "float", "complex"];

pub fn find_arithmetic_type_index(type_tag: &str) -> i32 {
    for (i, t) in ARITHMETIC_TYPES.iter().enumerate() {
        if type_tag == *t {
            return i as i32;
        }
    }
    -1
}
pub fn is_basis_arithmetic_type(x: &List) -> bool {
    find_arithmetic_type_index(&type_tag(x).to_string()) >= 0
}
// 将两个值的类型提升到统一的类型。
pub fn unify_arithmetic_types(a1: List, a2: List, arith: &ArithmeticContext) -> (List, List) {
    let a1_index = find_arithmetic_type_index(&type_tag(&a1).to_string());
    let a2_index = find_arithmetic_type_index(&type_tag(&a2).to_string());
    fn type_raise(x: &List, index_diff: i32, arith: &ArithmeticContext) -> List {
        if index_diff <= 0 {
            x.clone()
        } else {
            type_raise(&arith.raise(x), index_diff - 1, arith)
        }
    }
    let a1 = if a1_index < a2_index {
        type_raise(&a1, a2_index - a1_index, arith)
    } else {
        a1
    };
    let a2 = if a1_index > a2_index {
        type_raise(&a2, a1_index - a2_index, arith)
    } else {
        a2
    };
    (a1, a2)
}

#[derive(Clone)]
pub struct ArithmeticContext {
    // 这是所有的操作函数的表格
    pub optable: Rc<dyn Fn(&str) -> ClosureWrapper>,
    // 这是类型转换的表格
    pub coercion: List,
}
macro_rules! define_methods {
    ($($fn_name:ident, $op_name:expr, 2);* $(;)?) => {
        $(
            pub fn $fn_name(&self, x: &List, y: &List) -> List {
                self.apply_generic($op_name, &list![x.clone(), y.clone()]).unwrap()
            }
        )*
    };
    ($($fn_name:ident, $op_name:expr, 1);* $(;)?) => {
        $(
            pub fn $fn_name(&self, x: &List) -> List {
                self.apply_generic($op_name, &list![x.clone()]).unwrap()
            }
        )*
    };
}
impl ArithmeticContext {
    pub fn new() -> Self {
        ArithmeticContext {
            optable: make_table_2d(),
            coercion: List::Nil,
        }
    }
    pub fn get(&self, keys: List) -> Option<ClosureWrapper> {
        let lookup = (self.optable)("lookup");
        if let Some(closure) = lookup.call(&keys) {
            if let Ok(closure) = closure.try_as_basis_value::<ClosureWrapper>() {
                Some(closure.clone())
            } else {
                eprintln!(
                    "ArithmeticContext get failed! try_into_Closure failed for keys:{}",
                    keys
                );
                None
            }
        } else {
            None
        }
    }
    pub fn put(&self, key1: &'static str, key2: List, closure: ClosureWrapper) {
        let insert = (self.optable)("insert");
        if insert
            .call(&list![key1.to_listv(), key2.clone(), closure])
            .is_none()
        {
            panic!(
                "ArithmeticContext put failed! insert closure failed for keys:{}",
                list![key1, key2]
            )
        }
    }

    define_methods! {
        add, "add", 2;
        sub, "sub", 2;
        mul, "mul", 2;
        div, "div", 2;
        is_equal, "is_equal", 2;
    }

    define_methods! {
        is_equal_to_zero, "is_equal_to_zero", 1;
        negative, "negative", 1;
        abs, "abs", 1;
        numer, "numer", 1;
        denom, "denom", 1;
        real_part, "real_part", 1;
        imag_part, "imag_part", 1;
        magnitude, "magnitude", 1;
        angle, "angle", 1;
    }
    pub fn raise(&self, x: &List) -> List {
        // only raise for basis arith type and if x is not a complex number
        if !is_basis_arithmetic_type(x) || type_tag(x) == "complex".to_listv() {
            x.clone()
        } else {
            self.apply_generic(&"raise", &list![x.clone()]).unwrap()
        }
    }
    pub fn project(&self, x: &List) -> List {
        // only project for basis arith type and if x is not an integer
        if !is_basis_arithmetic_type(x) || type_tag(x) == "integer".to_listv() {
            x.clone()
        } else {
            self.apply_generic(&"project", &list![x.clone()]).unwrap()
        }
    }
    pub fn sqrt(&self, x: &List) -> List {
        assert!(
            is_basis_arithmetic_type(x) && type_tag(x) != "complex".to_listv(),
            "sqrt only for (integer, rational, float)"
        );
        self.apply_generic(&"sqrt", &list![x.clone()]).unwrap()
    }
    /// term_list support
    /// always return sparse term_list, as [sparse, [term]], not [sparse, term], use contents(head(tl)) to get first term
    pub fn first_term(&self, t: &List) -> List {
        self.apply_generic(&"first_term", &list![t.clone()])
            .unwrap()
    }
    pub fn rest_terms(&self, t: &List) -> List {
        self.apply_generic(&"rest_terms", &list![t.clone()])
            .unwrap()
    }
    pub fn adjoin_term(&self, t: &List, term_list: &List) -> List {
        self.apply_generic(&"adjoin_term", &list![t.clone(), term_list.clone()])
            .unwrap()
    }
    pub fn drop(&self, x: &List) -> List {
        // integer类型已无法继续drop
        self.drop_to_type(x, "integer".to_string())
    }
    pub fn drop_to_type(&self, x: &List, target_type: String) -> List {
        if type_tag(x) == target_type.clone().to_listv() || type_tag(x) == "integer".to_listv() {
            return x.clone();
        };
        let new_x = self.project(x);
        if self.is_equal(&self.raise(&new_x), &x) == true.to_listv() {
            self.drop_to_type(&new_x, target_type)
        } else {
            // 可能不是target_type，已无法继续drop
            x.clone()
        }
    }
    // coercion support
    pub fn put_coercion(
        &mut self,
        type1: &List,
        type2: &List,
        proc: ClosureWrapper,
    ) -> Option<List> {
        if self.get_coercion(type1, type2).is_none() {
            self.coercion = pair![
                list![type1.clone(), type2.clone(), proc],
                self.coercion.clone()
            ]
        }
        Some("done".to_listv())
    }
    pub fn get_coercion(&self, type1: &List, type2: &List) -> Option<ClosureWrapper> {
        fn get_type1(list_item: &List) -> List {
            list_item.head()
        }
        fn get_type2(list_item: &List) -> List {
            list_item.tail().head()
        }
        fn get_proc(list_item: &List) -> List {
            list_item.tail().tail().head()
        }
        fn get_coercion_iter(type1: &List, type2: &List, items: &List) -> Option<ClosureWrapper> {
            if items.is_empty() {
                None
            } else {
                let top = items.head();

                if get_type1(&top) == *type1 && get_type2(&top) == *type2 {
                    if let Ok(proc) = get_proc(&top).try_as_basis_value::<ClosureWrapper>() {
                        Some(proc.clone())
                    } else {
                        eprintln!(
                            "get_coercion_iter failed! try_into_Closure failed for keys:{}",
                            top
                        );
                        None
                    }
                } else {
                    get_coercion_iter(type1, type2, &items.tail())
                }
            }
        }
        get_coercion_iter(type1, type2, &self.coercion)
    }
    pub fn apply_generic(&self, op: &'static str, args: &List) -> Option<List> {
        apply_generic(&op.to_listv(), args, self)
    }
}

pub fn make_integer(x: i32, arith: &ArithmeticContext) -> List {
    if let Some(integer) = arith
        .get(list!["make", list!["integer"]])
        .expect("make_integer: arith.get(list![\"make\", list![\"integer\"]]) failed])")
        .call(&list![x])
    {
        integer
    } else {
        panic!("make_integer failed for x:{}", x)
    }
}
pub fn make_float(x: f64, arith: &ArithmeticContext) -> List {
    if let Some(float) = arith
        .get(list!["make", list!["float"]])
        .expect("make_float: arith.get(list![\"make\", list![\"float\"]]) failed])])")
        .call(&list![x])
    {
        float
    } else {
        panic!("make_float failed for x:{}", x)
    }
}
pub fn make_rational(n: List, d: List, arith: &ArithmeticContext) -> List {
    if let Some(rational) = arith
        .get(list!["make", list!["rational"]])
        .expect("make_rational: arith.get(list![\"make\", list![\"rational\"]]) failed])])])")
        .call(&list![n.clone(), d.clone()])
    {
        rational
    } else {
        panic!("make_rational failed for n:{}, d:{}", n, d)
    }
}
pub fn make_complex_from_real_imag(x: List, y: List, arith: &ArithmeticContext) -> List {
    if let Some(complex) = arith
        .get(list!["make_from_real_imag", list!["complex"]])
        .expect("make_complex_from_real_imag: arith.get(list![\"make_from_real_imag\", list![\"complex\"]]) failed])")
        .call(&list![x.clone(), y.clone()])
    {
        complex
    } else {
        panic!("make_complex_from_real_imag failed for x:{}, y:{}", x, y)
    }
}
pub fn make_complex_from_mag_ang(r: List, a: List, arith: &ArithmeticContext) -> List {
    if let Some(complex) = arith
        .get(list!["make_from_mag_ang", list!["complex"]])
        .expect("make_complex_from_mag_ang: arith.get(list![\"make_from_mag_ang\", list![\"complex\"]]) failed])").call(&list![r.clone(), a.clone()])
    {
        complex
    } else {
        panic!("make_complex_from_mag_ang failed for r:{}, a:{}", r, a)
    }
}
pub fn make_terms_from_sparse(term_list: &List, arith: &ArithmeticContext) -> List {
    if let Some(terms) = arith
       .get(list!["make_terms_from_sparse", list!["sparse"]])
       .expect("make_terms_from_sparse: arith.get(list![\"make_terms_from_sparse\", list![\"sparse\"]]) failed])")
       .call(&list![term_list.clone()])
    {
        terms
    } else {
        panic!("make_terms_from_sparse failed for term_list:{}", term_list)
    }
}
pub fn make_terms_from_dense(term_list: &List, arith: &ArithmeticContext) -> List {
    if let Some(terms) = arith
      .get(list!["make_terms_from_dense", list!["dense"]])
      .expect("make_terms_from_dense: arith.get(list![\"make_terms_from_dense\", list![\"dense\"]]) failed])")
      .call(&list![term_list.clone()])
    {
        terms
    } else {
        panic!("make_terms_from_dense failed for term_list:{}", term_list)
    }
}
pub fn make_polynomial_from_sparse(
    variable: &List,
    term_list: &List,
    arith: &ArithmeticContext,
) -> List {
    if let Some(polynomial) = arith
        .get(list!["make_polynomial_from_sparse", list!["polynomial"]])
        .expect("make_polynomial_from_sparse: arith.get(list![\"make_polynomial_from_sparse\", list![\"polynomial\"]]) failed])")
        .call(&list![variable.clone(), term_list.clone()])
    {
        polynomial
    } else {
        panic!("make_polynomial_from_sparse failed for variable:{}, term_list:{}", variable,term_list)
    }
}
pub fn make_polynomial_from_dense(
    variable: &List,
    term_list: &List,
    arith: &ArithmeticContext,
) -> List {
    if let Some(polynomial) = arith
        .get(list!["make_polynomial_from_dense", list!["polynomial"]])
        .expect("make_polynomial_from_dense: arith.get(list![\"make_polynomial_from_dense\", list![\"polynomial\"]]) failed])")
        .call(&list![variable.clone(), term_list.clone()])
    {
        polynomial
    } else {
        panic!("make_polynomial_from_dense failed for variable:{}, term_list:{}", variable,term_list)
    }
}
fn install_binary_op<T: fmt::Debug + Clone + 'static>(
    op_name: &'static str,
    tag_name: &'static str,
    op: impl Fn(T, T) -> List + 'static,
    arith: &ArithmeticContext,
) {
    let get_value = move |x: &List| {
        x.try_as_basis_value::<T>()
            .expect(&format!(
                "{} only supports {}, please construct {} with the correct type",
                tag_name,
                std::any::type_name::<T>(),
                tag_name
            ))
            .clone()
    };

    arith.put(
        op_name,
        list![tag_name, tag_name],
        ClosureWrapper::new(move |args: &List| {
            let (x, y) = (get_value(&args.head()), get_value(&args.tail().head()));
            Some(op(x, y))
        }),
    );
}
fn install_unary_op<T: fmt::Debug + Clone + 'static>(
    op_name: &'static str,
    tag_name: &'static str,
    op: impl Fn(T) -> List + 'static,
    arith: &ArithmeticContext,
) {
    let get_value = move |x: &List| {
        x.try_as_basis_value::<T>()
            .expect(&format!(
                "{} only supports {}, please construct {} with the correct type",
                tag_name,
                std::any::type_name::<T>(),
                tag_name
            ))
            .clone()
    };
    arith.put(
        op_name,
        list![tag_name],
        ClosureWrapper::new(move |args: &List| {
            let x = get_value(&args.head());
            Some(op(x))
        }),
    )
}
pub fn install_basic_numeric_type<T: fmt::Debug + Copy + Num + PartialOrd + 'static>(
    tag_name: &'static str,
    make_value: impl Fn(T) -> List + Clone + 'static,
    arith: &ArithmeticContext,
) -> Option<List> {
    let tag = |x| attach_tag(tag_name, &x);

    install_binary_op::<T>(
        "add",
        tag_name,
        {
            let make_value_ = make_value.clone();
            move |x: T, y: T| make_value_(x + y)
        },
        arith,
    );
    install_binary_op::<T>(
        "sub",
        tag_name,
        {
            let make_value_ = make_value.clone();
            move |x: T, y: T| make_value_(x - y)
        },
        arith,
    );
    install_binary_op::<T>(
        "mul",
        tag_name,
        {
            let make_value_ = make_value.clone();
            move |x: T, y: T| make_value_(x * y)
        },
        arith,
    );
    install_binary_op::<T>(
        "div",
        tag_name,
        {
            let make_value_ = make_value.clone();
            move |x: T, y: T| {
                if y == T::zero() {
                    panic!("{} divide by zero", tag_name)
                } else {
                    make_value_(x / y)
                }
            }
        },
        arith,
    );

    install_binary_op::<T>(
        "is_equal",
        tag_name,
        move |x: T, y: T| (x.to_listv() == y.to_listv()).to_listv(),
        arith,
    );
    install_unary_op::<T>(
        "is_equal_to_zero",
        tag_name,
        move |x: T| (x == T::zero()).to_listv(),
        arith,
    );
    install_unary_op::<T>(
        "negative",
        tag_name,
        {
            let make_value_ = make_value.clone();
            move |x: T| make_value_((T::zero() - T::one()) * x)
        },
        arith,
    );
    install_unary_op::<T>(
        "abs",
        tag_name,
        {
            let make_value_ = make_value.clone();
            move |x: T| {
                let abs = if x >= T::zero() {
                    x
                } else {
                    (T::zero() - T::one()) * x
                };
                make_value_(abs)
            }
        },
        arith,
    );

    install_unary_op::<T>("make", tag_name, move |x| tag(x.to_listv()), arith);

    Some("done".to_string().to_listv())
}
pub fn install_integer_package(arith: &ArithmeticContext) -> Option<List> {
    install_basic_numeric_type::<i32>(
        "integer",
        {
            let arith = arith.clone();
            move |x| make_integer(x, &arith)
        },
        arith,
    );
    arith.put("raise", list!["integer"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args| Some(make_rational(args.head(), 1.to_listv(), &arith)))
    });
    // sqrt integer
    arith.put("sqrt", list!["integer"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args| {
            let x = *(args
                .head()
                .try_as_basis_value::<i32>()
                .expect("sqrt integer: integer must be i32"));
            let x = arith.drop_to_type(
                &make_float((x as f64).sqrt(), &arith),
                "integer".to_string(),
            );
            // 返回值可能不是integer
            Some(x)
        })
    });
    Some("done".to_string().to_listv())
}
pub fn install_float_package(arith: &ArithmeticContext) -> Option<List> {
    install_basic_numeric_type::<f64>(
        "float",
        {
            let arith = arith.clone();
            move |x| make_float(x, &arith)
        },
        arith,
    );
    arith.put("raise", list!["float"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args| {
            Some(make_complex_from_real_imag(
                args.head(),
                0.0.to_listv(),
                &arith,
            ))
        })
    });
    // project real to rational
    arith.put("project", list!["float"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args| {
            let real = args.head();
            let real = real.try_as_basis_value::<f64>().unwrap();
            let (numer, denom) = float_to_fraction(*real, i32::MAX);
            Some(make_rational(numer.to_listv(), denom.to_listv(), &arith))
        })
    });
    // sqrt float
    arith.put("sqrt", list!["float"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args| {
            let x = *(args
                .head()
                .try_as_basis_value::<f64>()
                .expect("sqrt float: float must be f64"));
            Some(make_float(x.sqrt(), &arith))
        })
    });
    Some("done".to_string().to_listv())
}

pub fn install_rational_package(arith: &ArithmeticContext) -> Option<List> {
    let tag = |x| attach_tag("rational", &x);
    arith.put("make", list!["rational"], {
        let arith = arith.clone();
        let tag = tag.clone();
        ClosureWrapper::new(move |args| {
            let (n, d) = (args.head(), args.tail().head());
            assert!(
                type_tag(&n) != "float".to_listv() && type_tag(&d) != "float".to_listv(),
                "make rational: numer and denom must not float"
            );
            if arith.is_equal_to_zero(&d) == true.to_listv() {
                panic!("make rational: zero denominator");
            }

            if type_tag(&n) == "integer".to_listv() && type_tag(&d) == "integer".to_listv() {
                let (n, d) = (
                    n.try_as_basis_value::<i32>()
                        .expect("make rational with integer error"),
                    d.try_as_basis_value::<i32>()
                        .expect("make rational with integer error"),
                );
                let g = (*n).gcd(d);
                Some(tag(pair!(n / g, d / g)))
            } else {
                // may be complex or polynomial
                Some(tag(pair!(n.clone(), d.clone())))
            }
        })
    });
    arith.put(
        "numer",
        list!["rational"],
        ClosureWrapper::new(move |args| Some(args.head().head())),
    );
    arith.put(
        "denom",
        list!["rational"],
        ClosureWrapper::new(move |args| Some(args.head().tail())),
    );

    let extract_xy_numer_denom = {
        let (arith, tag) = (arith.clone(), tag.clone());
        move |args: &List| {
            // 使用 tag 函数重新附加数据类型标签：
            // apply_generic 在处理参数时会移除类型标签，
            // 这里通过 tag 函数重新为参数附加类型标签，以便后续操作能够识别数据类型。
            let (x, y) = (tag(args.head()), tag(args.tail().head()));
            let (numer_x, denom_x) = (arith.numer(&x), arith.denom(&x));
            let (numer_y, denom_y) = (arith.numer(&y), arith.denom(&y));
            (numer_x, denom_x, numer_y, denom_y)
        }
    };
    arith.put("add", list!["rational", "rational"], {
        let (arith, extract_xy_) = (arith.clone(), extract_xy_numer_denom.clone());
        ClosureWrapper::new(move |args| {
            let (n_x, d_x, n_y, d_y) = extract_xy_(args);
            let n = arith.add(&arith.mul(&n_x, &d_y), &arith.mul(&n_y, &d_x));
            let d = arith.mul(&d_x, &d_y);
            Some(make_rational(n, d, &arith))
        })
    });
    arith.put("sub", list!["rational", "rational"], {
        let (arith, extract_xy_) = (arith.clone(), extract_xy_numer_denom.clone());
        ClosureWrapper::new(move |args| {
            let (n_x, d_x, n_y, d_y) = extract_xy_(args);
            let n = arith.sub(&arith.mul(&n_x, &d_y), &arith.mul(&n_y, &d_x));
            let d = arith.mul(&d_x, &d_y);
            Some(make_rational(n, d, &arith))
        })
    });
    arith.put("mul", list!["rational", "rational"], {
        let (arith, extract_xy_) = (arith.clone(), extract_xy_numer_denom.clone());
        ClosureWrapper::new(move |args| {
            let (n_x, d_x, n_y, d_y) = extract_xy_(args);
            let n = arith.mul(&n_x, &n_y);
            let d = arith.mul(&d_x, &d_y);
            Some(make_rational(n, d, &arith))
        })
    });
    arith.put("div", list!["rational", "rational"], {
        let (arith, extract_xy_) = (arith.clone(), extract_xy_numer_denom.clone());
        ClosureWrapper::new(move |args| {
            let (n_x, d_x, n_y, d_y) = extract_xy_(args);
            let n = arith.mul(&n_x, &d_y);
            let d = arith.mul(&d_x, &n_y);
            Some(make_rational(n, d, &arith))
        })
    });
    arith.put("is_equal", list!["rational", "rational"], {
        let (arith, extract_xy_) = (arith.clone(), extract_xy_numer_denom.clone());
        ClosureWrapper::new(move |args| {
            let (n_x, d_x, n_y, d_y) = extract_xy_(args);
            Some(
                (arith.is_equal(&n_x, &n_y) == true.to_listv()
                    && arith.is_equal(&d_x, &d_y) == true.to_listv())
                .to_listv(),
            )
        })
    });
    arith.put("is_equal_to_zero", list!["rational"], {
        let (arith, tag) = (arith.clone(), tag.clone());
        ClosureWrapper::new(move |args| {
            // 调用链中有apply_generic的调用，需要使用 tag 函数重新附加数据类型标签
            let n = arith.numer(&tag(args.head()));
            Some((arith.is_equal_to_zero(&n) == true.to_listv()).to_listv())
        })
    });
    arith.put("negative", list!["rational"], {
        let (arith, tag) = (arith.clone(), tag.clone());
        ClosureWrapper::new(move |args| {
            // 调用链中有apply_generic的调用，需要使用 tag 函数重新附加数据类型标签
            let n = arith.numer(&tag(args.head()));
            let d = arith.denom(&tag(args.head()));
            Some(make_rational(arith.negative(&n), d, &arith))
        })
    });
    arith.put("raise", list!["rational"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args| {
            // 调用链中有apply_generic的调用，需要使用 tag 函数重新附加数据类型标签
            let n = arith.numer(&tag(args.head()));
            let d = arith.denom(&tag(args.head()));
            // try drop to integer
            let (n, d) = (arith.drop(&n), arith.drop(&d));
            if type_tag(&n) == "integer".to_listv() && type_tag(&d) == "integer".to_listv() {
                let n = n
                    .try_as_basis_value::<i32>()
                    .expect("raise rational with integer error");
                let d = d
                    .try_as_basis_value::<i32>()
                    .expect("raise rational with integer error");
                let f = (*n as f64) / (*d as f64);
                Some(make_float(f, &arith))
            } else {
                panic!(
                    "raise: rational to float error, not support {} to raise",
                    args
                );
            }
        })
    });
    // project rational to integer
    arith.put("project", list!["rational"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args| {
            // 调用链中有apply_generic的调用，需要使用 tag 函数重新附加数据类型标签
            let n = arith.numer(&tag(args.head()));
            let d = arith.denom(&tag(args.head()));
            // try drop to integer
            let (n, d) = (arith.drop(&n), arith.drop(&d));
            if type_tag(&n) == "integer".to_listv() && type_tag(&d) == "integer".to_listv() {
                let numer = *n.try_as_basis_value::<i32>().unwrap() as f64;
                let denom = *d.try_as_basis_value::<i32>().unwrap() as f64;
                let i = (numer / denom).floor() as i32;
                Some(make_integer(i, &arith))
            } else {
                eprintln!(
                    "project: rational to integer error: numer&denom only support integer found {}",
                    args
                );
                None
            }
        })
    });
    // sqrt rational
    arith.put("sqrt", list!["rational"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args| {
            // 调用链中有apply_generic的调用，需要使用 tag 函数重新附加数据类型标签
            let n = arith.numer(&tag(args.head()));
            let d = arith.denom(&tag(args.head()));
            // try drop to integer
            let (n, d) = (arith.drop(&n), arith.drop(&d));
            if type_tag(&n) == "integer".to_listv() && type_tag(&d) == "integer".to_listv() {
                let n = n
                    .try_as_basis_value::<i32>()
                    .expect("sqrt rational with integer error");
                let d = d
                    .try_as_basis_value::<i32>()
                    .expect("sqrt rational with integer error");
                let f = make_float(((*n as f64) / (*d as f64)).sqrt(), &arith);
                // 返回值可能不是rational
                Some(arith.drop_to_type(&f, "rational".to_string()))
            } else {
                panic!("sqrt rational error, not support {} to sqrt", args);
            }
        })
    });
    Some("done".to_string().to_listv())
}
// 将浮点数转换为分数（分子和分母）
// 使用连续分数法（Continued Fraction Method）
// # 参数
// - `x`: 待转换的浮点数
// - `max_denominator`: 分母的最大值，用于限制精度
// # 返回值
// 返回一个元组 `(numerator, denominator)`，分别是分子和分母
pub fn float_to_fraction(x: f64, max_denominator: i32) -> (i32, i32) {
    // 如果输入为负数，先处理符号
    let negative = x < 0.0; // 判断是否为负数
    let mut x = x.abs(); // 如果是负数，取绝对值处理

    // 初始化分子和分母
    let mut numer0: i32 = 0; // 分子 numer_{-1}
    let mut numer1: i32 = 1; // 分子 numer_0
    let mut denom0: i32 = 1; // 分母 denom_{-1}
    let mut denom1: i32 = 0; // 分母 denom_0

    // 当前的整数部分
    let mut a = x.floor() as i32; // 提取整数部分 \( a_0 = \lfloor x \rfloor \)

    while denom1 < max_denominator {
        // 更新分子和分母
        // let numer2 = a * numer1 + numer0; // \( numer_{n+1} = a_n \cdot numer_n + numer_{n-1} \)
        // let denom2 = a * denom1 + denom0; // \( denom_{n+1} = a_n \cdot denom_n + denom_{n-1} \)

        // 检查乘法和加法是否会溢出
        if let Some(numer2) = a.checked_mul(numer1).and_then(|v| v.checked_add(numer0)) {
            if let Some(denom2) = a.checked_mul(denom1).and_then(|v| v.checked_add(denom0)) {
                // 如果没有溢出，更新分子和分母
                if denom2 > max_denominator {
                    break;
                }
                numer0 = numer1;
                numer1 = numer2;
                denom0 = denom1;
                denom1 = denom2;
            } else {
                // 如果分母计算溢出，终止计算
                break;
            }
        } else {
            // 如果分子计算溢出，终止计算
            break;
        }

        // 更新小数部分
        x = x - a as f64; // 计算小数部分
        if x.abs().to_listv() == 0.0.to_listv() {
            // 如果小数部分接近 0，停止迭代
            break;
        }

        x = 1.0 / x; // \( x = \frac{1}{x} \)
        a = x.floor() as i32; // 提取新的整数部分
    }

    // 如果是负数，调整符号
    if negative {
        (numer1 * -1, denom1)
    } else {
        (numer1, denom1)
    }
}
pub fn install_rectangular_package(arith: &ArithmeticContext) -> Option<List> {
    let tag = |x: &List| attach_tag("rectangular", x);
    arith.put("make_from_real_imag", list!["rectangular"], {
        let tag = tag.clone();
        ClosureWrapper::new(move |args| {
            let (x, y) = (args.head(), args.tail().head());
            Some(tag(&pair!(x, y)))
        })
    });
    arith.put("make_from_mag_ang", list!["rectangular"], {
        let tag = tag.clone();
        ClosureWrapper::new(move |args| {
            let (r, a) = (args.head(), args.tail().head());
            if r.is_float_value() && a.is_float_value() {
                let r = r
                    .try_as_basis_value::<f64>()
                    .expect("complex: float type only support f64");
                let a = a
                    .try_as_basis_value::<f64>()
                    .expect("complex: float type only support f64");
                Some(tag(&pair![*r * a.cos(), *r * a.sin()]))
            } else {
                todo!("complex make_from_mag_ang rectangular Now only support f64")
            }
        })
    });

    arith.put("real_part", list!["rectangular"], {
        ClosureWrapper::new(move |args| Some(args.head().head()))
    });
    arith.put("imag_part", list!["rectangular"], {
        ClosureWrapper::new(move |args| Some(args.head().tail()))
    });

    let extract_real_imag = {
        let arith = arith.clone();
        move |arg: &List| {
            // 使用 tag 函数重新附加数据类型标签：
            // apply_generic 在处理参数时会移除类型标签，
            // 这里通过 tag 函数重新为参数附加类型标签，以便后续操作能够识别数据类型。
            let args = tag(arg);
            let (real_x, imag_x) = (arith.real_part(&args), arith.imag_part(&args));
            (real_x, imag_x)
        }
    };
    arith.put("magnitude", list!["rectangular"], {
        let extract = extract_real_imag.clone();
        let arith = arith.clone();
        ClosureWrapper::new(move |args| {
            let (real, imag) = extract(&args.head());
            if is_basis_arithmetic_type(&real)
                && type_tag(&real) != "complex".to_listv()
                && is_basis_arithmetic_type(&real)
                && type_tag(&real) != "complex".to_listv()
            {
                // sqrt only for (integer, rational, float)
                // (real*real + imag*imag).sqrt()
                let r2 = arith.mul(&real, &real);
                let i2 = arith.mul(&imag, &imag);
                let x = arith.add(&r2, &i2);
                Some(arith.drop(&arith.sqrt(&x)))
            } else {
                panic!("complex magnitude not support for {}", args);
            }
        })
    });
    arith.put("angle", list!["rectangular"], {
        let extract = extract_real_imag.clone();
        ClosureWrapper::new(move |args| {
            let (real, imag) = extract(&args.head());
            if real.is_float_value() && imag.is_float_value() {
                let r = real
                    .try_as_basis_value::<f64>()
                    .expect("complex: float type only support f64");
                let i = imag
                    .try_as_basis_value::<f64>()
                    .expect("complex: float type only support f64");
                Some((i.atan2(*r)).to_listv())
            } else {
                todo!("complex angle Now only support f64")
            }
        })
    });
    arith.put("is_equal", list!["rectangular", "rectangular"], {
        let arith = arith.clone();
        let extract = extract_real_imag.clone();
        ClosureWrapper::new(move |args| {
            let (x, y) = (args.head(), args.tail().head());
            let ((r_x, i_x), (r_y, i_y)) = (extract(&x), extract(&y));
            Some(
                (arith.is_equal(&r_x, &r_y) == true.to_listv()
                    && arith.is_equal(&i_x, &i_y) == true.to_listv())
                .to_listv(),
            )
        })
    });
    arith.put("is_equal_to_zero", list!["rectangular"], {
        let arith = arith.clone();
        let extract = extract_real_imag.clone();
        ClosureWrapper::new(move |args| {
            let (r, i) = extract(&args.head());
            Some(
                (arith.is_equal_to_zero(&r) == true.to_listv()
                    && arith.is_equal_to_zero(&i) == true.to_listv())
                .to_listv(),
            )
        })
    });
    Some("done".to_string().to_listv())
}

pub fn install_polar_package(arith: &ArithmeticContext) -> Option<List> {
    let tag = |x: &List| attach_tag("polar", x);
    arith.put("make_from_mag_ang", list!["polar"], {
        let tag = tag.clone();
        ClosureWrapper::new(move |args| {
            let (x, y) = (args.head(), args.tail().head());
            Some(tag(&pair!(x, y)))
        })
    });
    arith.put("make_from_real_imag", list!["polar"], {
        let tag = tag.clone();
        ClosureWrapper::new(move |args| {
            let (r, i) = (args.head(), args.tail().head());
            if r.is_float_value() && i.is_float_value() {
                let r = r
                    .try_as_basis_value::<f64>()
                    .expect("complex: float type only support f64")
                    .clone();
                let i = i
                    .try_as_basis_value::<f64>()
                    .expect("complex: float type only support f64")
                    .clone();
                Some(tag(&pair![(r * r + i * i).sqrt(), i.atan2(r)]))
            } else {
                todo!("complex make_from_real_imag polar Now only support f64")
            }
        })
    });

    arith.put("magnitude", list!["polar"], {
        ClosureWrapper::new(move |args| Some(args.head().head()))
    });
    arith.put("angle", list!["polar"], {
        ClosureWrapper::new(move |args| Some(args.head().tail()))
    });
    let extract_mag_ang = {
        let arith = arith.clone();
        move |arg: &List| {
            // 使用 tag 函数重新附加数据类型标签：
            // apply_generic 在处理参数时会移除类型标签，
            // 这里通过 tag 函数重新为参数附加类型标签，以便后续操作能够识别数据类型。
            let args = tag(arg);
            let (mag, ang) = (arith.magnitude(&args), arith.angle(&args));
            (mag, ang)
        }
    };
    arith.put("real_part", list!["polar"], {
        let extract = extract_mag_ang.clone();
        ClosureWrapper::new(move |args| {
            let (mag, ang) = extract(&args.head());
            if mag.is_float_value() && ang.is_float_value() {
                let m = mag
                    .try_as_basis_value::<f64>()
                    .expect("complex: float type only support f64")
                    .clone();
                let a = ang
                    .try_as_basis_value::<f64>()
                    .expect("complex: float type only support f64")
                    .clone();
                Some((m * a.cos()).to_listv())
            } else {
                todo!("complex real_part Now only support f64")
            }
        })
    });
    arith.put("imag_part", list!["polar"], {
        let extract = extract_mag_ang.clone();
        ClosureWrapper::new(move |args| {
            let (mag, ang) = extract(&args.head());
            if mag.is_float_value() && ang.is_float_value() {
                let m = mag
                    .try_as_basis_value::<f64>()
                    .expect("complex: float type only support f64")
                    .clone();
                let a = ang
                    .try_as_basis_value::<f64>()
                    .expect("complex: float type only support f64")
                    .clone();
                Some((m * a.sin()).to_listv())
            } else {
                todo!("complex imag_part Now only support f64")
            }
        })
    });
    arith.put("is_equal", list!["polar", "polar"], {
        let arith = arith.clone();
        let extract = extract_mag_ang.clone();
        ClosureWrapper::new(move |args| {
            let (x, y) = (args.head(), args.tail().head());
            let ((m_x, a_x), (m_y, a_y)) = (extract(&x), extract(&y));
            Some(
                (arith.is_equal(&m_x, &m_y) == true.to_listv()
                    && arith.is_equal(&a_x, &a_y) == true.to_listv())
                .to_listv(),
            )
        })
    });
    arith.put("is_equal_to_zero", list!["polar"], {
        let arith = arith.clone();
        let extract = extract_mag_ang.clone();
        ClosureWrapper::new(move |args| {
            let (m, _) = extract(&args.head());
            Some((arith.is_equal_to_zero(&m) == true.to_listv()).to_listv())
        })
    });
    Some("done".to_string().to_listv())
}

pub fn install_complex_package(arith: &ArithmeticContext) -> Option<List> {
    let tag = |x: &List| attach_tag("complex", x);
    arith.put("make_from_real_imag", list!["complex"], {
        let tag = tag.clone();
        let arith = arith.clone();
        ClosureWrapper::new(move |args| {
            let (x, y) = (args.head(), args.tail().head());
            if let Some(complex) = arith
                .get(list!["make_from_real_imag", list!["rectangular"]])
                .expect("make_from_real_imag rectangular failed get func")
                .call(&list![x.clone(), y.clone()])
            {
                Some(tag(&complex))
            } else {
                panic!("make_from_real_imag rectangular failed for args:{}", args)
            }
        })
    });
    arith.put("make_from_mag_ang", list!["complex"], {
        let tag = tag.clone();
        let arith = arith.clone();
        ClosureWrapper::new(move |args| {
            let (x, y) = (args.head(), args.tail().head());
            if let Some(complex) = arith
                .get(list!["make_from_mag_ang", list!["polar"]])
                .expect("make_from_mag_ang polar failed get func")
                .call(&list![x.clone(), y.clone()])
            {
                Some(tag(&complex))
            } else {
                panic!("make_from_mag_ang polar failed for args:{}", args)
            }
        })
    });
    arith.put("add", list!["complex", "complex"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args| {
            let (r1, i1) = (arith.real_part(&args.head()), arith.imag_part(&args.head()));
            let (r2, i2) = (
                arith.real_part(&args.tail().head()),
                arith.imag_part(&args.tail().head()),
            );
            let (r, i) = (arith.add(&r1, &r2), arith.add(&i1, &i2));
            Some(make_complex_from_real_imag(r, i, &arith))
        })
    });
    arith.put("sub", list!["complex", "complex"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args| {
            let (r1, i1) = (arith.real_part(&args.head()), arith.imag_part(&args.head()));
            let (r2, i2) = (
                arith.real_part(&args.tail().head()),
                arith.imag_part(&args.tail().head()),
            );
            let (r, i) = (arith.sub(&r1, &r2), arith.sub(&i1, &i2));
            Some(make_complex_from_real_imag(r, i, &arith))
        })
    });
    arith.put("mul", list!["complex", "complex"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args| {
            let (m1, a1) = (arith.magnitude(&args.head()), arith.angle(&args.head()));
            let (m2, a2) = (
                arith.magnitude(&args.tail().head()),
                arith.angle(&args.tail().head()),
            );
            let (m, a) = (arith.mul(&m1, &m2), arith.add(&a1, &a2));
            Some(make_complex_from_real_imag(m, a, &arith))
        })
    });
    arith.put("div", list!["complex", "complex"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args| {
            let (m1, a1) = (arith.magnitude(&args.head()), arith.angle(&args.head()));
            let (m2, a2) = (
                arith.magnitude(&args.tail().head()),
                arith.angle(&args.tail().head()),
            );
            let (m, a) = (arith.div(&m1, &m2), arith.sub(&a1, &a2));
            Some(make_complex_from_real_imag(m, a, &arith))
        })
    });
    arith.put("negative", list!["complex"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args| {
            let (r, i) = (arith.real_part(&args.head()), arith.imag_part(&args.head()));
            Some(make_complex_from_real_imag(
                arith.negative(&r),
                arith.negative(&i),
                &arith,
            ))
        })
    });
    arith.put("is_equal", list!["complex", "complex"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args| Some(arith.is_equal(&args.head(), &args.tail().head())))
    });
    arith.put("is_equal_to_zero", list!["complex"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args| Some(arith.is_equal_to_zero(&args.head())))
    });
    arith.put("real_part", list!["complex"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args| Some(arith.real_part(&args)))
    });
    arith.put("imag_part", list!["complex"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args| Some(arith.imag_part(&args)))
    });
    arith.put("magnitude", list!["complex"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args| Some(arith.magnitude(&args)))
    });
    arith.put("angle", list!["complex"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args| Some(arith.angle(&args)))
    });
    // project complex to real
    arith.put("project", list!["complex"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args| {
            let real = arith.real_part(args);
            let real = if type_tag(&real).to_listv() == "integer".to_listv() {
                *(real.try_as_basis_value::<i32>().unwrap()) as f64
            } else if type_tag(&real) == "float".to_listv() {
                *real.try_as_basis_value::<f64>().unwrap()
            } else if type_tag(&real) == "rational".to_listv() {
                *(arith.raise(&real).try_as_basis_value::<f64>().unwrap())
            } else {
                eprintln!("project complex to real only support basis arithmetic type");
                return None;
            };

            Some(make_float(real, &arith))
        })
    });
    Some("done".to_string().to_listv())
}
pub fn is_variable(x: &List) -> bool {
    x.is_string_value()
}
pub fn is_same_variable(v1: &List, v2: &List) -> bool {
    is_variable(v1) && is_variable(v2) && v1 == v2
}
// representation of poly
pub fn make_poly(variable: List, term_list: List) -> List {
    pair![variable, term_list]
}
/// (x, [sparse, term_list]) -> x
pub fn variable(p: &List) -> List {
    p.head()
}
// (x, [sparse, term_list]) -> [sparse, term_list]
pub fn term_list(p: &List) -> List {
    p.tail()
}
// representation of terms and term lists
pub fn order(term: &List) -> List {
    term.head()
}
pub fn coeff(term: &List) -> List {
    term.tail().head()
}
pub fn make_term(order: List, coeff: List) -> List {
    list![order, coeff]
}
// term_list [sparse, [term...]]
pub fn is_empty_term_list(term_list: &List) -> bool {
    contents(term_list).is_empty()
}
pub fn make_empty_term_list(arith: &ArithmeticContext) -> List {
    make_terms_from_sparse(&List::Nil, arith)
}
// term_list [term...]
pub fn rest_terms(term_list: &List) -> List {
    if term_list.is_empty() {
        List::Nil
    } else {
        term_list.tail()
    }
}
/// arith.first_term always return sparse term_list, as [sparse, [term]], not [sparse, term]
/// pure_first_term([sparse, [term]]) -> term
pub fn pure_first_term(first_term: &List) -> List {
    contents(first_term).head()
}
pub fn pretty_polynomial(p: &List, arith: &ArithmeticContext) -> String {
    // (polynomial, x, sparse, (2, 4), (1, 3), (0, 7.0))
    fn iter(term_list: &List, arith: &ArithmeticContext) -> String {
        let t1 = arith.first_term(term_list); // (sparse, (2, 4))
        let order1 = order(&contents(&t1).head());
        let coeff1 = coeff(&contents(&t1).head());
        let coeff1_str = if type_tag(&coeff1) == "polynomial".to_listv() {
            pretty_polynomial(&coeff1, arith)
        } else {
            coeff1.to_string()
        };
        if order1 == 0.to_listv() {
            coeff1_str
        } else {
            format!(
                "{}x^{} + {}",
                coeff1_str,
                order1,
                iter(&arith.rest_terms(term_list), arith)
            )
        }
    }
    let tl = term_list(&contents(&p)); // (sparse, (2, 4), (1, 3), (0, 7.0))
    if contents(&tl).is_empty() {
        format!("({}:{})", type_tag(p), contents(&tl))
    } else {
        format!("({}:{})", type_tag(p), iter(&tl, arith))
    }
}
pub fn install_sparse_terms_package(arith: &ArithmeticContext) -> Option<List> {
    fn first_term(term_list: &List) -> List {
        term_list.head()
    }
    fn adjoin_term(term: List, term_list: List, arith: &ArithmeticContext) -> List {
        if arith.is_equal_to_zero(&coeff(&term)) == true.to_listv() {
            term_list.clone()
        } else {
            pair![term.clone(), term_list.clone()]
        }
    }
    fn tag(x: &List) -> List {
        attach_tag("sparse", x)
    }
    arith.put("first_term", list!["sparse"], {
        ClosureWrapper::new(move |args: &List| {
            // always return sparse term_list, as [sparse, [term]], not [sparse, term]
            // head: term_list ;
            let p = args.head();
            let term_list = list![first_term(&p)];
            Some(tag(&term_list))
        })
    });
    arith.put("rest_terms", list!["sparse"], {
        ClosureWrapper::new(move |args: &List| {
            // head: term_list ;
            let p = args.head();
            let term_list = rest_terms(&p);
            Some(tag(&term_list))
        })
    });
    arith.put("adjoin_term", list!["sparse", "sparse"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args: &List| {
            let (t1, l) = (args.head().head(), args.tail().head());
            Some(tag(&adjoin_term(t1, l, &arith)))
        })
    });
    arith.put("make_terms_from_sparse", list!["sparse"], {
        ClosureWrapper::new(move |args: &List| {
            let p = args.head();
            Some(tag(&p))
        })
    });
    Some("done".to_string().to_listv())
}
pub fn install_dense_terms_package(arith: &ArithmeticContext) -> Option<List> {
    fn order_dense(term_list: &List) -> List {
        if term_list.is_empty() {
            0.to_listv()
        } else {
            ((term_list.length() - 1) as i32).to_listv()
        }
    }
    fn coeff_dense(term_list: &List) -> List {
        if term_list.is_empty() {
            0.to_listv()
        } else {
            term_list.head()
        }
    }
    fn first_term_dense(term_list: &List) -> List {
        make_term(order_dense(term_list), coeff_dense(term_list))
    }
    fn adjoin_term_dense(term: List, term_list: List, arith: &ArithmeticContext) -> List {
        if arith.is_equal_to_zero(&coeff(&term)) == true.to_listv() {
            term_list
        } else {
            let term_order = order(&term);
            let term_coeff = coeff(&term);
            let term_list_order = order_dense(&term_list);
            assert!(
                term_order.get_basis_value() >= term_list_order.get_basis_value(),
                "adjoin_term: term_list is ordered. The new term must maintain this order by having an order >= all existing terms."
            );
            if term_order == term_list_order {
                // new term_list
                if term_list.is_empty() {
                    list![term_coeff]
                } else {
                    pair![arith.add(&term_list.head(), &term_coeff), term_list.tail()]
                }
            } else {
                // term_order > term_list_order
                adjoin_term_dense(term, pair![0, term_list], arith)
            }
        }
    }
    fn tag(x: &List) -> List {
        attach_tag("dense", x)
    }
    fn sparse_tag(x: &List) -> List {
        attach_tag("sparse", x)
    }
    arith.put("first_term", list!["dense"], {
        ClosureWrapper::new(move |args: &List| {
            // always return sparse term_list, as [sparse, [term]], not [sparse, term]
            let p = args.head();
            let term_list = list![first_term_dense(&p)];
            Some(sparse_tag(&term_list))
        })
    });
    arith.put("rest_terms", list!["dense"], {
        ClosureWrapper::new(move |args: &List| {
            // head: term_list ;
            let p = args.head();
            let term_list = rest_terms(&p);
            Some(tag(&term_list))
        })
    });
    arith.put("adjoin_term", list!["sparse", "dense"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args: &List| {
            let (t1, l) = (args.head().head(), args.tail().head());
            Some(tag(&adjoin_term_dense(t1, l, &arith)))
        })
    });
    arith.put("make_terms_from_dense", list!["dense"], {
        ClosureWrapper::new(move |args: &List| {
            let p = args.head();
            Some(tag(&p))
        })
    });
    Some("done".to_string().to_listv())
}

pub fn install_polynomial_package(arith: &ArithmeticContext) -> Option<List> {
    fn add_terms(l1: &List, l2: &List, arith: &ArithmeticContext) -> List {
        if is_empty_term_list(l1) {
            l2.clone()
        } else if is_empty_term_list(l2) {
            l1.clone()
        } else {
            let t1 = arith.first_term(&l1);
            let (order1, coeff1) = (order(&pure_first_term(&t1)), coeff(&pure_first_term(&t1)));
            let t2 = arith.first_term(&l2);
            let (order2, coeff2) = (order(&pure_first_term(&t2)), coeff(&pure_first_term(&t2)));

            if order1.get_basis_value() > order2.get_basis_value() {
                arith.adjoin_term(&t1, &add_terms(&arith.rest_terms(&l1), l2, arith))
            } else if order1.get_basis_value() < order2.get_basis_value() {
                arith.adjoin_term(&t2, &add_terms(l1, &arith.rest_terms(&l2), &arith))
            } else {
                let first_term = make_terms_from_sparse(
                    &list![make_term(order1, arith.add(&coeff1, &coeff2))],
                    arith,
                );

                arith.adjoin_term(
                    &first_term,
                    &add_terms(&arith.rest_terms(l1), &arith.rest_terms(l2), &arith),
                )
            }
        }
    }
    fn mul_term_by_all_terms(t1: &List, l: &List, arith: &ArithmeticContext) -> List {
        if is_empty_term_list(l) {
            make_empty_term_list(arith) //[sparse, List::Nil]
        } else {
            let (order1, coeff1) = (order(&pure_first_term(&t1)), coeff(&pure_first_term(&t1)));
            let t2 = arith.first_term(&l);
            let (order2, coeff2) = (order(&pure_first_term(&t2)), coeff(&pure_first_term(&t2)));
            let first_term = make_terms_from_sparse(
                &list![make_term(
                    arith.add(&order1, &order2),
                    arith.mul(&coeff1, &coeff2)
                )],
                arith,
            );
            arith.adjoin_term(
                &first_term,
                &mul_term_by_all_terms(&t1, &arith.rest_terms(&l), &arith),
            )
        }
    }
    fn mul_terms(l1: &List, l2: &List, arith: &ArithmeticContext) -> List {
        if is_empty_term_list(l1) {
            make_empty_term_list(arith) //[sparse, List::Nil]
        } else {
            add_terms(
                &mul_term_by_all_terms(&arith.first_term(l1), l2, &arith),
                &mul_terms(&arith.rest_terms(l1), l2, &arith),
                &arith,
            )
        }
    }
    fn negative_terms(l: &List, arith: &ArithmeticContext) -> List {
        if is_empty_term_list(l) {
            make_empty_term_list(arith) //[sparse, List::Nil]
        } else {
            let t1 = arith.first_term(l);
            let (order1, coeff1) = (order(&pure_first_term(&t1)), coeff(&pure_first_term(&t1)));
            let first_term =
                make_terms_from_sparse(&list![make_term(order1, arith.negative(&coeff1))], arith);

            arith.adjoin_term(&first_term, &negative_terms(&arith.rest_terms(l), &arith))
        }
    }
    fn add_poly(p1: &List, p2: &List, arith: &ArithmeticContext) -> List {
        if is_same_variable(&variable(p1), &variable(p2)) {
            make_poly(
                variable(p1),
                add_terms(&term_list(p1), &term_list(p2), arith),
            )
        } else {
            panic!(
                "{} Polys not in same var -- ADD-POLY",
                list![p1.clone(), p2.clone()]
            )
        }
    }

    fn mul_poly(p1: &List, p2: &List, arith: &ArithmeticContext) -> List {
        if is_same_variable(&variable(p1), &variable(p2)) {
            make_poly(
                variable(p1),
                mul_terms(&term_list(p1), &term_list(p2), arith),
            )
        } else {
            panic!(
                "{} Polys not in same var -- MUL-POLY",
                list![p1.clone(), p2.clone()]
            )
        }
    }
    fn is_equal_to_zero(term_list: &List, arith: &ArithmeticContext) -> List {
        if is_empty_term_list(term_list) {
            true.to_listv()
        } else {
            // [sparse [term]]-> term
            let t = pure_first_term(&arith.first_term(term_list)); // [sparse [term]]
            if arith.is_equal_to_zero(&coeff(&t)) == false.to_listv() {
                false.to_listv()
            } else {
                is_equal_to_zero(&arith.rest_terms(term_list), &arith)
            }
        }
    }

    fn tag(x: &List) -> List {
        attach_tag("polynomial", x)
    }

    arith.put("add", list!["polynomial", "polynomial"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args: &List| {
            let (p1, p2) = (args.head(), args.tail().head());
            Some(tag(&add_poly(&p1, &p2, &arith)))
        })
    });

    arith.put("mul", list!["polynomial", "polynomial"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args: &List| {
            let (p1, p2) = (args.head(), args.tail().head());
            Some(tag(&mul_poly(&p1, &p2, &arith)))
        })
    });

    arith.put("is_equal_to_zero", list!["polynomial"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args: &List| {
            let term_list = term_list(&args.head());
            Some(is_equal_to_zero(&term_list, &arith))
        })
    });
    arith.put("negative", list!["polynomial"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args: &List| {
            let variable = variable(&args.head());
            let term_list = term_list(&args.head());
            Some(tag(&make_poly(
                variable,
                negative_terms(&term_list, &arith),
            )))
        })
    });
    arith.put("sub", list!["polynomial", "polynomial"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args: &List| {
            let (p1, p2) = (args.head(), args.tail().head());
            // 需要补上被apply_generic剥去的标签
            let (p1, p2) = (tag(&p1), arith.negative(&tag(&p2)));
            Some(arith.add(&p1, &p2))
        })
    });
    arith.put("make_polynomial_from_dense", list!["polynomial"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args: &List| {
            let (variable, term_list) = (args.head(), args.tail().head());
            let term_list = if type_tag(&term_list) == "sparse".to_listv() {
                eprintln!("warning: try to make dense terms, but found sparse terms arg");
                term_list
            } else if type_tag(&term_list) == "dense".to_listv() {
                term_list
            } else {
                make_terms_from_sparse(&term_list, &arith)
            };
            Some(tag(&make_poly(variable, term_list)))
        })
    });
    arith.put("make_polynomial_from_sparse", list!["polynomial"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args: &List| {
            let (variable, term_list) = (args.head(), args.tail().head());
            let term_list = if type_tag(&term_list) == "sparse".to_listv() {
                term_list
            } else if type_tag(&term_list) == "dense".to_listv() {
                eprintln!("warning: try to make sparse terms, but found dense terms arg");
                term_list
            } else {
                make_terms_from_sparse(&term_list, &arith)
            };
            Some(tag(&make_poly(variable, term_list)))
        })
    });
    Some("done".to_string().to_listv())
}
