use sicp_rs::prelude::*;
use std::collections::HashSet;

fn count_pairs(x: List, visited: &mut HashSet<u64>) -> i32 {
    let x_unique_id = x.unique_id(); // unique_id 实际上底层数据指针

    if x.is_pair() && !visited.contains(&x_unique_id) {
        visited.insert(x_unique_id);
        1 + count_pairs(x.head(), visited) + count_pairs(x.tail(), visited)
    } else {
        0
    }
}
fn main() {
    let x = pair!("a", "b");
    println!(
        "pairs: {}",
        count_pairs(pair![1, pair![2, pair![3, 4]]], &mut HashSet::new())
    );
    println!(
        "pairs: {}",
        count_pairs(pair![1, pair![x.clone(), pair![3, 4]]], &mut HashSet::new())
    );
    println!(
        "pairs: {}",
        count_pairs(
            pair![1, pair![x.clone(), pair![x.clone(), 4]]],
            &mut HashSet::new()
        )
    );
    println!(
        "pairs: {}",
        count_pairs(
            pair![1, pair![x.clone(), pair![x.clone(), x.clone()]]],
            &mut HashSet::new()
        )
    );
    println!(
        "pairs: {}",
        count_pairs(
            pair![x.clone(), pair![x.clone(), pair![x.clone(), x.clone()]]],
            &mut HashSet::new()
        )
    );

    let y = pair![1, 2];
    y.set_tail(y.clone());
    println!("never return: {}", count_pairs(y, &mut HashSet::new()));
}
// pairs: 3
// pairs: 4
// pairs: 4
// pairs: 4
// pairs: 4
// never return: 1