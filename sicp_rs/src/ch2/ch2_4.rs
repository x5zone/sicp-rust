use crate::prelude::*;

use super::ch2_5::get_coercion;
pub fn attach_tag(tag: &str, contents: &List) -> List {
    // Only Support f64
    if contents.is_value() && contents.try_as_basis_value::<f64>().is_ok() {
        contents.clone()
    } else {
        pair!(tag.to_string(), contents.clone())
    }
}

pub fn type_tag(datum: &List) -> List {
    // Only Support f64
    if datum.is_value() && datum.try_as_basis_value::<f64>().is_ok() {
        "javascript_number".to_string().to_listv()
    } else if datum.is_pair() {
        datum.head()
    } else {
        panic!("bad tagged datum -- TYPE-TAG")
    }
}
pub fn contents(datum: &List) -> List {
    // Only Support f64
    if datum.is_value() && datum.try_as_basis_value::<f64>().is_ok() {
        datum.clone()
    } else if datum.is_pair() {
        datum.tail()
    } else {
        panic!("bad tagged datum -- CONTENTS")
    }
}
pub fn apply_generic(
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
    // 为兼容历史代码与习题，不改变函数签名的同时，支持coercion
    // op 结构为 pair![list!["coercion", coercion], op]
    let (op, coercion) =
        if op.is_pair() && op.head().is_pair() && op.head().head() == "coercion".to_listv() {
            (op.tail(), op.head().tail().head())
        } else {
            (op.clone(), List::Nil)
        };
    let op_cloned = op.clone();
    let type_tags = args.map(|x| type_tag(x));
    let type_tags_cloned = type_tags.clone();
    let op = get(list![op.clone(), type_tags]);
    if let Some(op) = op {
        if let Ok(op) = op.try_as_basis_value::<ClosureWrapper>() {
            return op.call(&args.map(|x| contents(x)));
        } else {
            None
        }
    } else {
        if args.length() == 2 {
            let type1 = type_tags_cloned.head();
            let type2 = type_tags_cloned.tail().head();
            if type1 == type2 {
                panic!("No method for these types op:{}, args:{}", op_cloned, args);
            }
            let a1 = args.head();
            let a2 = args.tail().head();
            let t1_to_t2 = get_coercion(&type1, &type2, &coercion);
            let t2_to_t1 = get_coercion(&type2, &type1, &coercion);
            if t1_to_t2.is_some() {
                let t1_to_t2 = t1_to_t2.unwrap();
                let t1_to_t2 = t1_to_t2.try_as_basis_value::<ClosureWrapper>().unwrap();
                let a1 = t1_to_t2.call(&list![a1.clone()]);
                apply_generic(&op_cloned, &list![a1.unwrap(), a2], get)
            } else if t2_to_t1.is_some() {
                let t2_to_t1 = t2_to_t1.unwrap();
                let t2_to_t1 = t2_to_t1.try_as_basis_value::<ClosureWrapper>().unwrap();
                let a2 = t2_to_t1.call(&list![a2.clone()]);
                apply_generic(&op_cloned, &list![a1, a2.unwrap()], get)
            } else {
                panic!("No method for these types op:{}, args:{}", op_cloned, args);
            }
        } else {
            panic!("No method for these types op:{}, args:{}", op_cloned, args);
        }
    }
}
