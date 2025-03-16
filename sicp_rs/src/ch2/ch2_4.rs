use crate::prelude::*;
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
    let op_cloned = op.clone();
    let type_tags = args.map(|x| type_tag(x));
    let op = get(list![op.clone(), type_tags]);
    if let Some(op) = op {
        if let Ok(op) = op.try_as_basis_value::<ClosureWrapper>() {
            return op.call(&args.map(|x| contents(x)));
        }
    }
    panic!("No method for these types op:{}, args:{}", op_cloned, args);
}
