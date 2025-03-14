use crate::prelude::*;

/// 2.3.2 实例： 符号求导
pub fn is_number(exp: &List) -> bool {
    exp.is_value() && exp.get_basis_value().as_ref().is_number()
}
pub fn is_variable(exp: &List) -> bool {
    exp.is_value() && exp.get_basis_value().as_ref().is_string()
}
pub fn is_same_variable(exp1: &List, exp2: &List) -> bool {
    is_variable(exp1) && exp1 == exp2
}
