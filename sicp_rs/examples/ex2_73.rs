use std::rc::Rc;

use num::Num;
use num::pow::Pow;
use sicp_rs::ch2::ch2_3::{
    addend, augend, base, exponent, is_number, is_same_variable, is_variable, make_exp,
    make_product, make_sum, multiplicand, multiplier,
};
use sicp_rs::ch3::ch3_3::make_table_2d;
use sicp_rs::prelude::*;

/// 获取表达式的运算符
fn operator(exp: &List) -> List {
    exp.head()
}

/// 获取表达式的操作数
fn _operands(exp: &List) -> List {
    //ch2.56节代码中直接对exp取操作数,故若使用历史代码并同时使用operands,会多取一次tail
    exp.tail()
}

/// 通用的求导函数，基于数据导向分派
fn deriv<T: Num + Clone + std::fmt::Debug + 'static>(
    exp: &List,
    variable: &List,
    optable: &impl Fn(List) -> Option<List>,
) -> List {
    if is_number(exp) {
        T::zero().to_listv()
    } else if is_variable(exp) {
        if is_same_variable(exp, variable) {
            T::one().to_listv()
        } else {
            T::zero().to_listv()
        }
    } else {
        // 从操作符表中获取对应的求导规则
        if let Some(op) = optable(list!["deriv", operator(exp).clone()]) {
            if let Ok(op) = op.try_as_basis_value::<ClosureWrapper>() {
                if let Some(result) = op.call(&list![exp.clone(), variable.clone()]) {
                    return result;
                }
            }
        }
        panic!("unknown operator -- DERIV, exp {}", exp)
    }
}

/// 求和表达式的求导规则
fn deriv_sum<T: Num + Clone + std::fmt::Debug + 'static>(
    exp: &List,
    variable: &List,
    optable: impl Fn(List) -> Option<List>,
) -> List {
    make_sum::<T>(
        deriv::<T>(&addend(exp), variable, &optable),
        deriv::<T>(&augend(exp), variable, &optable),
    )
}

/// 乘积表达式的求导规则
fn deriv_product<T: Num + Clone + std::fmt::Debug + 'static>(
    exp: &List,
    variable: &List,
    optable: impl Fn(List) -> Option<List>,
) -> List {
    make_sum::<T>(
        make_product::<T>(
            multiplier(exp),
            deriv::<T>(&multiplicand(exp), variable, &optable),
        ),
        make_product::<T>(
            deriv::<T>(&multiplier(exp), variable, &optable),
            multiplicand(exp),
        ),
    )
}

/// 幂表达式的求导规则
fn deriv_exp<T: Num + Clone + std::fmt::Debug + Pow<T, Output = T> + 'static>(
    exp: &List,
    variable: &List,
    optable: impl Fn(List) -> Option<List>,
) -> List {
    make_product::<T>(
        make_product::<T>(
            exponent(exp),
            make_exp::<T>(
                base(exp),
                make_sum::<T>(exponent(exp), (T::zero() - T::one()).to_listv()),
            ),
        ),
        deriv::<T>(&base(exp), variable, &optable),
    )
}

fn main() {
    // 创建操作符表
    let optable: Rc<dyn Fn(&str) -> ClosureWrapper> = make_table_2d();
    let op_cloned = optable.clone();
    let get = move |args: List| optable("lookup").call(&args);
    let put = move |args: List| op_cloned("insert").call(&args);

    // 安装求导规则
    let install_rule = |op: String, rule: ClosureWrapper| {
        put(list!["deriv", op, rule]);
    };

    // 安装求和规则
    let get_cloned = get.clone();
    install_rule(
        "+".to_string(),
        ClosureWrapper::new(move |args: &List| {
            Some(deriv_sum::<f64>(
                &args.head(),
                &args.tail().head(),
                &get_cloned,
            ))
        }),
    );

    // 安装乘积规则
    let get_cloned = get.clone();
    install_rule(
        "*".to_string(),
        ClosureWrapper::new(move |args: &List| {
            Some(deriv_product::<f64>(
                &args.head(),
                &args.tail().head(),
                &get_cloned,
            ))
        }),
    );

    // 安装幂运算规则
    let get_cloned = get.clone();
    install_rule(
        "**".to_string(),
        ClosureWrapper::new(move |args: &List| {
            Some(deriv_exp::<f64>(
                &args.head(),
                &args.tail().head(),
                &get_cloned,
            ))
        }),
    );

    // 测试求导规则
    let exp1 = list!("*", list!("*", "x", "y"), list!("+", "x", 4.0));
    println!(
        "{}",
        deriv::<f64>(&exp1, &"x".to_listv(), &get).pretty_print()
    );

    let exp2 = list!("**", "x", list!("+", "y", 3.0));
    println!(
        "{}",
        deriv::<f64>(&exp2, &"x".to_listv(), &get).pretty_print()
    );

    let exp3 = list!("**", "x", "n");
    println!(
        "{}",
        deriv::<f64>(&exp3, &"x".to_listv(), &get).pretty_print()
    );
}
// Output
// ("+", ("*", "x", "y"), ("*", "y", ("+", "x", 4.0)))
// ("*", ("+", "y", 3.0), ("**", "x", ("+", ("+", "y", 3.0), -1.0)))
// ("*", "n", ("**", "x", ("+", "n", -1.0)))
