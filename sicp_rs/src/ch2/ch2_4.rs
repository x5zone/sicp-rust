use crate::prelude::*;

use super::ch2_5::ArithmeticContext;

pub fn attach_tag(tag: &str, contents: &List) -> List {
    pair!(tag.to_string(), contents.clone())
}

pub fn type_tag(datum: &List) -> List {
    if datum.is_pair() {
        datum.head()
    } else {
        panic!("bad tagged datum -- TYPE-TAG")
    }
}

pub fn contents(datum: &List) -> List {
    if datum.is_pair() {
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
    if let Some(func) = arith.get(list![op.clone(), type_tags]) {
        func.call(&args.map(|x| contents(x)))
    } else {
        panic!(
            "apply_generic no method for these types op:{}, args:{}",
            op, args
        )
    }
}