use sicp_rs::ch2::ch2_5::ArithmeticContext;
use sicp_rs::ch2::ch2_5::{attach_tag, contents, type_tag};

use sicp_rs::prelude::*;
// 通用型操作：根据操作符和参数调用对应的函数
fn apply_generic(op: &List, args: &List, arith: &ArithmeticContext) -> Option<List> {
    let args = if args.head().is_pair() && args.head().head().is_pair() {
        // 处理可能由于apply_generic导致的嵌套列表
        args.flatmap(|x| x.clone())
    } else {
        args.clone()
    };
    println!("apply generic op:{}, args:{}", op, args);
    let type_tags = args.map(|x| type_tag(x));
    if let Some(op) = arith.get(list![op.clone(), type_tags]) {
        op.call(&args.map(|x| contents(x)))
    } else {
        panic!(
            "apply_generic no method for these types op:{}, args:{}",
            op, args
        )
    }
}
fn real_part(z: &List, arith: &ArithmeticContext) -> List {
    apply_generic(&"real_part".to_listv(), &list![z.clone()], arith).unwrap()
}
fn imag_part(z: &List, arith: &ArithmeticContext) -> List {
    apply_generic(&"imag_part".to_listv(), &list![z.clone()], arith).unwrap()
}
fn magnitude(z: &List, arith: &ArithmeticContext) -> List {
    println!("magnitude {}", z);
    apply_generic(&"magnitude".to_listv(), &list![z.clone()], arith).unwrap()
}
fn install_rectangular_package(arith: &ArithmeticContext) -> Option<List> {
    let tag = |x: &List| attach_tag("rectangular", x);
    arith.put("make_from_real_imag", list!["rectangular"], {
        let tag = tag.clone();
        ClosureWrapper::new(move |args| {
            let (x, y) = (args.head(), args.tail().head());
            Some(tag(&pair!(x, y)))
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
        let tag = tag.clone();
        move |arg: &List| {
            // 使用 tag 函数重新附加数据类型标签：
            // apply_generic 在处理参数时会移除类型标签，
            // 这里通过 tag 函数重新为参数附加类型标签，以便后续操作能够识别数据类型。
            let args = tag(arg);
            let (real_x, imag_x) = (real_part(&args, &arith), imag_part(&args, &arith));
            (real_x, imag_x)
        }
    };
    arith.put("magnitude", list!["rectangular"], {
        let extract = extract_real_imag.clone();
        ClosureWrapper::new(move |args| {
            let (real, imag) = extract(&args.head());
            if real.is_float_value() && imag.is_float_value() {
                let r = real.try_as_basis_value::<f64>().unwrap();
                let i = imag.try_as_basis_value::<f64>().unwrap();
                Some((r * r + i * i).sqrt().to_listv())
            } else {
                panic!("Now only support f64")
            }
        })
    });
    Some("done".to_string().to_listv())
}

fn install_complex_package(arith: &ArithmeticContext) -> Option<List> {
    let tag = |x| attach_tag("complex", &x);
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
                Some(tag(complex))
            } else {
                panic!("make_from_real_imag rectangular failed for args:{}", args)
            }
        })
    });

    Some("done".to_string().to_listv())
}
fn make_complex_from_real_imag(x: List, y: List, arith: &ArithmeticContext) -> List {
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

fn main() {
    // 创建通用算术包上下文
    let arith = ArithmeticContext::new();

    println!("{:?}", install_rectangular_package(&arith));
    println!("{:?}", install_complex_package(&arith));
    let a = make_complex_from_real_imag(3.0.to_listv(), 4.0.to_listv(), &arith);
    println!("{}", a);

    if true {
        let magnitude_wrapper = ClosureWrapper::new({
            let arith = arith.clone();
            move |x: &List| Some(magnitude(x, &arith))
        });
        arith.put("magnitude", list!["complex"], magnitude_wrapper);
    } else {
        println!(
            "apply_generic no method for these types op:magnitude, args:((complex, (rectangular, (3.0, 4.0))), Nil)"
        )
    }

    println!("{}", magnitude(&a, &arith))
}
