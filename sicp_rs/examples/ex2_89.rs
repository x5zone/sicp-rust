use sicp_rs::{
    ch2::ch2_5::{
        attach_tag, coeff, contents, install_arithmetic_package, is_empty_term_list, is_same_variable, make_integer, make_poly, make_term, order, rest_terms, term_list, type_tag, variable, ArithmeticContext
    },
    prelude::*,
};
// make_poly,variable,term_list shared
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
    list![order_dense(term_list), coeff_dense(term_list)]
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
fn add_terms_dense(l1: &List, l2: &List, arith: &ArithmeticContext) -> List {
    if is_empty_term_list(&l1) {
        l2.clone()
    } else if is_empty_term_list(&l2) {
        l1.clone()
    } else {
        let t1 = first_term_dense(&l1);
        let t2 = first_term_dense(&l2);
        if order(&t1).get_basis_value() > order(&t2).get_basis_value() {
            let x = adjoin_term_dense(
                t1.clone(),
                add_terms_dense(&rest_terms(&l1), l2, &arith),
                &arith,
            );
            x
        } else if order(&t1).get_basis_value() < order(&t2).get_basis_value() {
            let x = adjoin_term_dense(
                t2.clone(),
                add_terms_dense(l1, &rest_terms(&l2), &arith),
                &arith,
            );
            x
        } else {
            let x = adjoin_term_dense(
                make_term(order(&t1), arith.add(&coeff(&t1), &coeff(&t2))),
                add_terms_dense(&rest_terms(&l1), &rest_terms(&l2), &arith),
                &arith,
            );
            x
        }
    }
}
pub fn mul_term_by_all_terms_dense(t1: &List, l: &List, arith: &ArithmeticContext) -> List {
    if is_empty_term_list(&l) {
        List::Nil
    } else {
        let t2 = first_term_dense(&l);
        adjoin_term_dense(
            make_term(
                arith.add(&order(&t1), &order(&t2)),
                arith.mul(&coeff(&t1), &coeff(&t2)),
            ),
            mul_term_by_all_terms_dense(t1, &rest_terms(&l), &arith),
            &arith,
        )
    }
}
pub fn mul_terms_dense(l1: &List, l2: &List, arith: &ArithmeticContext) -> List {
    if is_empty_term_list(&l1) {
        List::Nil
    } else {
        add_terms_dense(
            &mul_term_by_all_terms_dense(&first_term_dense(&l1), &l2, &arith),
            &mul_terms_dense(&rest_terms(&l1), &l2, &arith),
            &arith,
        )
    }
}
pub fn pretty_polynomial_dense(p: &List) -> String {
    fn iter(term_list: &List) -> String {
        let t1 = first_term_dense(term_list);
        let order1 = order(&t1);
        let coeff1 = coeff(&t1);
        let coeff1_str = if type_tag(&coeff1) == "dense".to_listv() {
            pretty_polynomial_dense(&coeff1)
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
                iter(&rest_terms(term_list))
            )
        }
    }
    if term_list(&contents(&p)).is_empty() {
        format!("({}:{})", type_tag(p), term_list(&contents(p)))
    } else {
        format!("({}:{})", type_tag(p), iter(&term_list(&contents(p))))
    }
}

pub fn make_polynomial_from_dense(
    variable: &List,
    term_list: &List,
    arith: &ArithmeticContext,
) -> List {
    if let Some(polynomial) = arith
        .get(list!["make_polynomial_from_dense", list!["dense"]])
        .expect("make_polynomial_from_dense: arith.get(list![\"make\", list![\"polynomial\"]]) failed])")
        .call(&list![variable.clone(), term_list.clone()])
    {
        polynomial
    } else {
        panic!("make_polynomial_from_dense failed for variable:{}, term_list:{}", variable,term_list)
    }
}
pub fn install_polynomial_dense_package(arith: &ArithmeticContext) -> Option<List> {
    fn add_poly(p1: &List, p2: &List, arith: &ArithmeticContext) -> List {
        if is_same_variable(&variable(p1), &variable(p2)) {
            make_poly(
                variable(p1),
                add_terms_dense(&term_list(p1), &term_list(p2), &arith),
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
                mul_terms_dense(&term_list(p1), &term_list(p2), &arith),
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
            // may be (polynomial:0x^2 + 0x^1 + 0)
            let t = first_term_dense(term_list);
            if arith.is_equal_to_zero(&coeff(&t)) == false.to_listv() {
                false.to_listv()
            } else {
                is_equal_to_zero(&rest_terms(term_list), &arith)
            }
        }
    }
    fn negative_terms(l: &List, arith: &ArithmeticContext) -> List {
        if is_empty_term_list(&l) {
            List::Nil
        } else {
            let t1 = first_term_dense(&l);
            let t1 = make_term(order(&t1), arith.negative(&coeff(&t1)));
            adjoin_term_dense(t1, negative_terms(&rest_terms(&l), &arith), &arith)
        }
    }
    fn tag(x: &List) -> List {
        attach_tag("dense", x)
    }

    arith.put("add", list!["dense", "dense"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args: &List| {
            let (p1, p2) = (args.head(), args.tail().head());
            Some(tag(&add_poly(&p1, &p2, &arith)))
        })
    });

    arith.put("mul", list!["dense", "dense"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args: &List| {
            let (p1, p2) = (args.head(), args.tail().head());
            Some(tag(&mul_poly(&p1, &p2, &arith)))
        })
    });
    arith.put(
        "make_polynomial_from_dense",
        list!["dense"],
        ClosureWrapper::new(move |args: &List| {
            let (variable, term_list) = (args.head(), args.tail().head());
            Some(tag(&make_poly(variable, term_list)))
        }),
    );
    arith.put("is_equal_to_zero", list!["dense"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args: &List| {
            let term_list = term_list(&args.head());
            Some(is_equal_to_zero(&term_list, &arith))
        })
    });
    arith.put("negative", list!["dense"], {
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
    arith.put("sub", list!["dense", "dense"], {
        let arith = arith.clone();
        ClosureWrapper::new(move |args: &List| {
            let (p1, p2) = (args.head(), args.tail().head());
            // 需要补上被apply_generic剥去的标签
            let (p1, p2) = (tag(&p1), arith.negative(&tag(&p2)));
            Some(arith.add(&p1, &p2))
        })
    });
    Some("done".to_string().to_listv())
}

fn main() {
    // 初始化算术上下文
    let arith = ArithmeticContext::new();
    install_arithmetic_package(&arith);
    install_polynomial_dense_package(&arith);

    // 创建两个稠密多项式
    let p1 = make_polynomial_from_dense(
        &"x".to_listv(),
        &list![
            // x^4 + 2x^3 + 0x^2 + 3x^1 + -2x^0
            make_integer(1, &arith),  // x^4
            make_integer(2, &arith),  // x^3
            make_integer(0, &arith),  // x^2
            make_integer(3, &arith),  // x^1
            make_integer(-2, &arith), // x^0
        ],
        &arith,
    );

    let p2 = make_polynomial_from_dense(
        &"x".to_listv(),
        &list![5, 0, -1, 4], // 5x^3 + 0x^2 -x^1 + 4
        &arith,
    );
    // 打印多项式
    println!("p1 = {}", pretty_polynomial_dense(&p1));
    println!("p2 = {}", pretty_polynomial_dense(&p2));

    // 测试加法
    let p_add = arith.add(&p1, &p2);
    println!("p1 + p2 = {}", pretty_polynomial_dense(&p_add)); // (dense:1x^4 + 7x^3 + 0x^2 + 2x^1 + 2)

    // 测试减法
    let p_sub = arith.sub(&p1, &p2);
    println!("p1 - p2 = {}", pretty_polynomial_dense(&p_sub)); // p1 * p2 = (dense:5x^7 + 10x^6 + -1x^5 + 17x^4 + -2x^3 + -3x^2 + 14x^1 + -8)
    // 测试乘法
    let p_mul = arith.mul(&p1, &p2);
    println!("p1 * p2 = {}", pretty_polynomial_dense(&p_mul));

    // 测试负号操作
    let p_neg = arith.negative(&p1);
    println!("-p1 = {}", pretty_polynomial_dense(&p_neg));
}
