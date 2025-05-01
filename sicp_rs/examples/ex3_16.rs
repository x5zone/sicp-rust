use sicp_rs::prelude::*;

fn count_pairs(x: List) -> i32 {
    if x.is_pair() {
        1 + count_pairs(x.head()) + count_pairs(x.tail())
    } else {
        0
    }
}
fn main() {
    let x = pair!("a", "b");
    println!("pairs: {}", count_pairs(pair![1, pair![2, pair![3, 4]]]));
    println!(
        "pairs: {}",
        count_pairs(pair![1, pair![x.clone(), pair![3, 4]]])
    );
    println!(
        "pairs: {}",
        count_pairs(pair![1, pair![x.clone(), pair![x.clone(), 4]]])
    );
    println!(
        "pairs: {}",
        count_pairs(pair![1, pair![x.clone(), pair![x.clone(), x.clone()]]])
    );
    println!(
        "pairs: {}",
        count_pairs(pair![
            x.clone(),
            pair![x.clone(), pair![x.clone(), x.clone()]]
        ])
    );

    let y = pair![1,2];
    y.set_tail(y.clone());
    // println!("never return: {}", count_pairs(y));
}
// pairs: 3
// pairs: 4
// pairs: 5
// pairs: 6
// pairs: 7