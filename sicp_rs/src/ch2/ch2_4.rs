use crate::prelude::*;
pub fn attach_tag(tag : &str, contents : &List) -> List {
    pair!(tag.to_string(), contents.clone())
}

pub fn type_tag(datum : &List) -> List {
    if datum.is_pair() {
        datum.head()
    }else {
        panic!("bad tagged datum -- TYPE-TAG")
    }
}
pub fn contents(datum : &List) -> List {
    if datum.is_pair() {
        datum.tail()
    }else {
        panic!("bad tagged datum -- CONTENTS")
    }
}
pub fn apply_generic(
    op: &List,
    args: &List,
    get: impl Fn(List) -> Option<List> + 'static,
) -> Option<List> {
    let op_cloned = op.clone();
    let type_tags = type_tag(args);
    let op = get(list![op.clone(), type_tags]);
    if let Some(op) = op {
        if let Ok(op) = op.try_as_basis_value::<ClosureWrapper>() {
            return op.call(&contents(args));
        }
    }
    panic!("No method for these types op:{}, args:{}", op_cloned, args);
}