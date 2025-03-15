use crate::prelude::*;
use num::Num;
use num::traits::Pow;

/// 2.3.2 实例： 符号求导
pub fn is_number(exp: &List) -> bool {
    exp.is_value() && exp.is_number_value()
}
pub fn is_variable(exp: &List) -> bool {
    exp.is_value() && exp.is_string_value()
}
pub fn is_same_variable(exp1: &List, exp2: &List) -> bool {
    is_variable(exp1) && exp1 == exp2
}

pub fn number_equal(exp: &List, num: &List) -> bool {
    is_number(exp) && exp == num
}

pub fn is_sum(x: &List) -> bool {
    x.is_pair() && x.head() == "+".to_listv()
}
pub fn addend(s: &List) -> List {
    s.tail().head()
}
pub fn augend(s: &List) -> List {
    s.tail().tail().head()
}
pub fn make_sum<T: Num + Clone + std::fmt::Debug + 'static>(a1: List, a2: List) -> List {
    if a1 == T::zero().to_listv() {
        a2
    } else if a2 == T::zero().to_listv() {
        a1
    } else if is_number(&a1) && is_number(&a2) {
        (a1.try_as_basis_value::<T>().unwrap().clone()
            + a2.try_as_basis_value::<T>().unwrap().clone())
        .to_listv()
    } else {
        list!["+", a1, a2]
    }
}
pub fn is_product(x: &List) -> bool {
    x.is_pair() && x.head() == "*".to_listv()
}
pub fn multiplier(p: &List) -> List {
    p.tail().head()
}
pub fn multiplicand(p: &List) -> List {
    p.tail().tail().head()
}
pub fn make_product<T: Num + Clone + std::fmt::Debug + 'static>(m1: List, m2: List) -> List {
    if m1 == T::zero().to_listv() || m2 == T::zero().to_listv() {
        T::zero().to_listv()
    } else if m1 == T::one().to_listv() {
        m2
    } else if m2 == T::one().to_listv() {
        m1
    } else if is_number(&m1) && is_number(&m2) {
        return (m1.try_as_basis_value::<T>().unwrap().clone()
            * m2.try_as_basis_value::<T>().unwrap().clone())
        .to_listv();
    } else {
        list!["*", m1, m2]
    }
}
pub fn is_exp(x: &List) -> bool {
    x.is_pair() && x.head() == "**".to_listv()
}
pub fn base(e: &List) -> List {
    e.tail().head()
}
pub fn exponent(e: &List) -> List {
    e.tail().tail().head()
}
pub fn make_exp<T: Num + Clone + std::fmt::Debug + Pow<T, Output = T> + 'static>(
    b: List,
    e: List,
) -> List {
    if e == T::zero().to_listv() {
        T::one().to_listv()
    } else if e == T::one().to_listv() {
        b
    } else if is_number(&b) && is_number(&e) {
        let base = b.try_as_basis_value::<T>().unwrap().clone();
        let exp = e.try_as_basis_value::<T>().unwrap().clone();
        Pow::pow(base, exp).to_listv()
    } else {
        list!["**", b, e]
    }
}
