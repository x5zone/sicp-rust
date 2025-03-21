use std::rc::Rc;

use sicp_rs::ch2::ch2_5::put_coercion;
use sicp_rs::ch3::ch3_3::make_table_2d;
use sicp_rs::prelude::*;
fn test_transform() {
    // 创建函数表格
    let func_table = make_table_2d();
    let func_table_cloned = func_table.clone();
    let put_func = move |args: List| func_table_cloned("insert").call(&args);

    // 定义多个函数签名[(type1,type5,type3),(type3,type4,type5),(type5,type5,type5)]
    {
        put_func(list![
            "complex_func",
            list!["type1", "type5", "type3"],
            ClosureWrapper::new(move |args: &List| {
                let result = format!(
                    "complex_func(type1, type5, type3) called with args: {}",
                    args
                );
                Some(result.to_string().to_listv())
            })
        ]);

        put_func(list![
            "complex_func",
            list!["type3", "type4", "type5"],
            ClosureWrapper::new(move |args: &List| {
                let result = format!(
                    "complex_func(type3, type4, type5) called with args: {}",
                    args
                );
                Some(result.to_string().to_listv())
            })
        ]);

        put_func(list![
            "complex_func",
            list!["type5", "type5", "type5"],
            ClosureWrapper::new(move |args: &List| {
                let result = format!(
                    "complex_func(type5, type5, type5) called with args: {}",
                    args
                );
                Some(result.to_string().to_listv())
            })
        ]);
    }

    // 创建类型强制表格[(type1->type2),(type2->type3),(type1->type4),(type4->type5),(type3->type5),(type1->type5),(type5->type2),(type1->type3)]
    let mut coercion = List::Nil;
    {
        coercion = put_coercion(
            &"type1".to_listv(),
            &"type2".to_listv(),
            ClosureWrapper::new(|args: &List| {
                let value = args.head();
                Some(format!("{} type1->type2", value).to_string().to_listv())
            }),
            &coercion,
        );

        coercion = put_coercion(
            &"type2".to_listv(),
            &"type3".to_listv(),
            ClosureWrapper::new(|args: &List| {
                let value = args.head();
                Some(format!("{} type2->type3", value).to_string().to_listv())
            }),
            &coercion,
        );

        coercion = put_coercion(
            &"type1".to_listv(),
            &"type4".to_listv(),
            ClosureWrapper::new(|args: &List| {
                let value = args.head();
                Some(format!("{} type1->type4", value).to_string().to_listv())
            }),
            &coercion,
        );

        coercion = put_coercion(
            &"type4".to_listv(),
            &"type5".to_listv(),
            ClosureWrapper::new(|args: &List| {
                let value = args.head();
                Some(format!("{} type4->type5", value).to_string().to_listv())
            }),
            &coercion,
        );

        coercion = put_coercion(
            &"type3".to_listv(),
            &"type5".to_listv(),
            ClosureWrapper::new(|args: &List| {
                let value = args.head();
                Some(format!("{} type3->type5", value).to_string().to_listv())
            }),
            &coercion,
        );

        coercion = put_coercion(
            &"type1".to_listv(),
            &"type5".to_listv(),
            ClosureWrapper::new(|args: &List| {
                let value = args.head();
                Some(format!("{} type1->type5", value).to_string().to_listv())
            }),
            &coercion,
        );
        coercion = put_coercion(
            &"type5".to_listv(),
            &"type2".to_listv(),
            ClosureWrapper::new(|args: &List| {
                let value = args.head();
                Some(format!("{} type5->type2", value).to_string().to_listv())
            }),
            &coercion,
        );
        coercion = put_coercion(
            &"type1".to_listv(),
            &"type3".to_listv(),
            ClosureWrapper::new(|args: &List| {
                let value = args.head();
                Some(format!("{} type1->type3", value).to_string().to_listv())
            }),
            &coercion,
        );
    }

    // 准备输入参数(type1,type2,type1)
    let input_args = list![
        pair!("arg1".to_listv(), "type1".to_listv()),
        pair!("arg2".to_listv(), "type2".to_listv()),
        pair!("arg3".to_listv(), "type1".to_listv())
    ];

    // 获取函数签名
    let func_name = "complex_func".to_string().to_listv();
    let func_types = get_func_argtypes(&func_name, input_args.length(), func_table.clone());

    // 执行参数转换
    let (transformed_args, selected_func) = transform(&input_args, func_types, &coercion);

    // 打印结果
    if let (Some(args), Some(func)) = (transformed_args, selected_func) {
        println!("Selected function: {}", func);
        println!("Transformed args: {}", args);

        // 调用选中的函数
        let func_closure = func.try_as_basis_value::<ClosureWrapper>().unwrap();
        let result = func_closure.call(&args).unwrap();
        println!("Function result: {}", result);
    } else {
        println!("No suitable function found.");
    }
}
// 处理所有函数签名，并返回成本最低的转换结果。
fn transform(input_args: &List, func_types: List, coercion: &List) -> (Option<List>, Option<List>) {
    let mut min_cost = i32::MAX;
    let mut best_results = None;
    let mut best_func = None;
    let mut func_types = func_types.clone();
    // func_types: list[pair[list[type1,type2,type3...],some_func],...]
    while func_types.is_empty() == false {
        let fun = func_types.head();

        let func = fun.tail();
        let (cost, flag, results) = transform_argtypes(&input_args, fun, &coercion, List::Nil, 0);
        if flag == true && cost < min_cost {
            min_cost = cost;
            println!("min arg tranforms: {}", min_cost);
            best_results = Some(results);
            best_func = Some(func);
        }
        func_types = func_types.tail();
    }
    return (best_results, best_func);
}
// 处理单个函数签名，并返回成本和转换后的参数列表。
fn transform_argtypes(
    input_args: &List,
    func_argtypes: List,
    coercion: &List,
    results: List,
    cost: i32,
) -> (i32, bool, List) {
    // input args: list[pair(arg1,type1),pair(arg2,type2),...]
    // func_types: pair[list[type1,type2,type3...],some_func]
    if input_args.is_empty() {
        // 所有参数都已匹配，则匹配成功。
        return (cost, true, results.clone());
    }

    let type_x = input_args.head().tail();

    // 当前参数是否满足函数的参数类型?
    if type_x == func_argtypes.head().head() {
        // results: list[pair(type1,val1),pair(type2,val2)...].append(list[pair(type_x,val_x)])
        let results = results.append(&list![pair!(type_x, input_args.head().head())]);
        // input_args: list[pair(arg2,type2),...]
        let input_args = input_args.tail();
        // func_types: pair[list[type2,type3...],some_func]
        let func_argtypes = list![func_argtypes.head().tail(), func_argtypes.tail()];
        let (cost, flag, results) =
            transform_argtypes(&input_args, func_argtypes, &coercion, results, cost);
        return (cost, flag, results.clone());
    } else {
        // 当前参数不匹配，尝试转换参数类型，可能需要经过多次转换。
        use std::collections::HashSet;
        let mut visited: HashSet<String> = HashSet::new(); // 用来记录访问过的节点
        fn dfs(
            source: &List,
            source_val: List,
            target: &List,
            visited: &mut HashSet<String>,
            coercion: &List,
            cost: i32,
        ) -> (i32, Option<List>) {
            if source == target {
                return (cost, Some(pair![source.clone(), source_val]));
            }

            if visited.contains(&source.to_string()) {
                return (cost, None);
            }

            visited.insert(source.to_string());
            // argtypes: list[list[type_y,trans_to_type_y_func],list[type_z,trans_to_type_z_func],...]
            let mut argtypes = get_type_coercion(source, coercion);
            while !argtypes.is_empty() {
                let type_y = argtypes.head().head();
                let trans_to_type_y_func = argtypes.head().tail().head();
                let new_val = trans_to_type_y_func
                    .try_as_basis_value::<ClosureWrapper>()
                    .unwrap();
                let new_val = new_val.call(&list![source_val.clone()]).unwrap(); // 转换次数+1，cost+1
                let results = dfs(&type_y, new_val, target, visited, coercion, cost + 1);
                if results.1.is_some() {
                    return results;
                }
                argtypes = argtypes.tail();
            }
            return (cost, None);
        }
        let (new_cost, trans_arg) = dfs(
            &type_x,
            input_args.head().head(),
            &func_argtypes.head().head(),
            &mut visited,
            &coercion,
            cost,
        );
        if let Some(trans_arg) = trans_arg {
            let trans_type = trans_arg.head();
            let trans_val = trans_arg.tail();
            // results: list[pair(type1,val1),pair(type2,val2)...].append(list[pair(trans_type,trans_val)])
            let results = results.append(&list![pair!(trans_type, trans_val)]);
            // input_args: list[pair(arg2,type2),...]
            let input_args = input_args.tail();
            // func_types: list[list[type2,type3...],some_func]
            let func_argtypes = list![func_argtypes.head().tail(), func_argtypes.tail()];
            let (cost, flag, results) =
                transform_argtypes(&input_args, func_argtypes, &coercion, results, new_cost);
            return (cost, flag, results.clone());
        } else {
            return (cost, false, List::Nil);
        }
    }
}
// 获取函数的所有可能参数类型列表
fn get_func_argtypes(
    func_name: &List,
    args_len: usize,
    func_map: Rc<dyn Fn(&str) -> ClosureWrapper>,
) -> List {
    let assoc = move |args: List| func_map("assoc").call(&args);
    let args = assoc(list![func_name.clone()]);
    let msg = format!("get_func_argtypes: func_name {} not found", func_name);
    args.expect(&msg)
        .tail()
        .filter(|args| args.head().length() == args_len)
}

// 获取参数可转换类型列表
fn get_type_coercion(type_x: &List, coercion: &List) -> List {
    coercion
        .filter(|x| {
            //let type1 = x.head();
            x.head() == *type_x
        })
        .map(|x| {
            //let type1 = x.head();
            let type2 = x.tail().head();
            let proc = x.tail().tail().head();
            list![type2, proc]
        })
}

fn main() {
    test_transform()
}
fn _test_get_func_argtypes() {
    let optable = make_table_2d();

    let op_cloned = optable.clone();
    let put = move |args: List| op_cloned("insert").call(&args);
    put(list![
        "some_func",
        list!["type1", "type2", "type3"],
        ClosureWrapper::new(move |_: &List| {
            Some("some_func(type1,type2,type3)".to_string().to_listv())
        })
    ]);
    put(list![
        "some_func",
        list!["type1", "type2", "type3", "type4"],
        ClosureWrapper::new(move |_: &List| {
            Some("some_func(type1,type2,type3,type4)".to_string().to_listv())
        })
    ]);
    put(list![
        "some_func",
        list!["type2", "type5", "type4"],
        ClosureWrapper::new(move |_: &List| {
            Some("some_func(type2,type5,type4)".to_string().to_listv())
        })
    ]);
    println!(
        "{}",
        get_func_argtypes(&"some_func".to_string().to_listv(), 3, optable.clone())
    )
    //((("type2", ("type5", ("type4", Nil))), A closure wrapped in ClosureWrapper), ((("type1", ("type2", ("type3", Nil))), A closure wrapped in ClosureWrapper), Nil))
}
fn _test_get_type_coercion() {
    let coercion = put_coercion(
        &"type1".to_listv(),
        &"type2".to_listv(),
        ClosureWrapper::new(|_| Some("type1 to type2".to_string().to_listv())),
        &List::Nil,
    );
    let coercion = put_coercion(
        &"type2".to_listv(),
        &"type3".to_listv(),
        ClosureWrapper::new(|_| Some("type2 to type3".to_string().to_listv())),
        &coercion,
    );
    let coercion = put_coercion(
        &"type1".to_listv(),
        &"type4".to_listv(),
        ClosureWrapper::new(|_| Some("type1 to type4".to_string().to_listv())),
        &coercion,
    );
    let coercion = put_coercion(
        &"type2".to_listv(),
        &"type5".to_listv(),
        ClosureWrapper::new(|_| Some("type2 to type5".to_string().to_listv())),
        &coercion,
    );
    let coercion = put_coercion(
        &"type4".to_listv(),
        &"type5".to_listv(),
        ClosureWrapper::new(|_| Some("type4 to type5".to_string().to_listv())),
        &coercion,
    );
    println!("{}", get_type_coercion(&"type1".to_listv(), &coercion))
    //(("type4", (A closure wrapped in ClosureWrapper, Nil)), (("type2", (A closure wrapped in ClosureWrapper, Nil)), Nil))
}
