use std::collections::HashSet;

use sicp_rs::prelude::*;

fn has_cycle(x: List) -> bool {
    fn iter(a: List, visited: &mut HashSet<u64>) -> bool {
        if a.is_pair() {
            // println!("a unique: {}", a.unique_id());
            // if a.head().is_value() {
            //     println!("a head value {}", a.head())
            // }
            // if a.tail().is_value() {
            //     println!("a tail value {}", a.tail())
            // }
            if visited.contains(&a.unique_id()) {
                return true;
            }
            visited.insert(a.unique_id());

            (
                // 若head也是pair，尝试遍历并查看是否会构成环路。
                // 为避免误判共享子结构，构建新的visited，并从此节点开始，若能重新回到该节点，即为有环。
                a.head().is_pair() && {
                let mut new_visited = HashSet::new();
                new_visited.insert(a.unique_id());
                iter(a.head(), &mut new_visited)
            }) || (a.tail().is_pair() && iter(a.tail(), visited))
            // 习题其实仅考察单链表，以下即为习题预期解答。
            // iter(a.tail(), visited)
        } else {
            false
        }
    }
    iter(x, &mut HashSet::new())
}

fn main() {
    let x = pair!["a", "b"];
    let y = pair![x.clone(), x.clone()];
    println!("shared substructure has_cycle(y): {}", has_cycle(y));
    let y = pair![x.clone(), pair![x.clone(), x.clone()]];
    println!("shared substructure has_cycle(y): {}", has_cycle(y));
    let y = pair![x.clone(), pair![x.clone(), pair![x.clone(), x.clone()]]];
    println!("shared substructure has_cycle(y): {}", has_cycle(y));
    let y = pair![
        x.clone(),
        pair![x.clone(), pair![x.clone(), pair![x.clone(), x.clone()]]]
    ];
    println!("shared substructure has_cycle(y): {}", has_cycle(y));

    let x = list![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let y = x.tail().tail().tail().tail();
    let z = x.tail().tail().tail().tail().tail().tail().tail();
    println!("y: {}\nz: {}", y, z);
    z.set_tail(y.clone());
    println!("cycle in tail has_cycle(x): {}", has_cycle(x.clone()));

    let x = list![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let y = x.tail().tail().tail().tail();
    let z = x.tail().tail().tail().tail().tail().tail().tail();
    println!("y: {}\nz: {}", y, z);
    z.set_head(y.clone());
    println!("cycle in head has_cycle(x): {}", has_cycle(z.head()));
    // 1 2 3 4 5 6 7 pointer
    //         |        |
    //         |--------|
    println!("cycle in head has_cycle(x): {}", has_cycle(x.clone()));
    // println!("{}", x.tail().tail().tail().tail().tail().tail().tail()); // will infinite loop
    // println!(
    //     "{}",
    //     x.tail().tail().tail().tail().tail().tail().tail().tail()
    // );
}
