use sicp_rs::prelude::*;

fn has_cycle(x: List) -> bool {
    fn iter(fast: &List, slow: &List) -> bool {
        if !fast.is_pair() || !slow.is_pair() {
            return false;
        }
        // fast有可前进方向
        if !(fast.tail().is_pair() || (fast.head().is_pair() && fast.head().tail().is_pair())) {
            return false;
        }
        if fast.unique_id() == slow.unique_id() {
            return true;
        }
        // 仅可通过递归方式实现。若通过while循环方式，在head为pair时，指针会出现两个可前进方向。
        // 而通过递归方式实现，在两个可前进方向时，可自然的进行分叉。
        (
            // fast head && slow head is pair
            fast.head().is_pair()
                && slow.head().is_pair()
                && fast.head().tail().is_pair()
                && iter(&fast.head().tail(), &slow.head())
        ) || (
            // fast head is pair
            fast.head().is_pair()
                && fast.head().tail().is_pair()
                && iter(&fast.head().tail(), &slow.tail())
        ) || (fast.tail().is_pair() && iter(&fast.tail().tail(), &slow.tail()))
        // 习题预期的解答如下：
        // (fast.tail().is_pair() && iter(&fast.tail().tail(), &slow.tail()))
    }
    x.is_pair() && iter(&x.tail(), &x)
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
    //println!("{}", x.tail().tail().tail().tail().tail().tail().tail()); // will infinite loop
    // println!(
    //     "{}",
    //     x.tail().tail().tail().tail().tail().tail().tail().tail()
    // );
}
