fn make_accumulator(init: i32) -> impl FnMut(i32) -> i32 {
    let mut sum = init;
    move |x| {
        sum += x;
        sum
    }
}
fn main() {
    let mut a = make_accumulator(5);
    println!("{}", a(10));
    println!("{}", a(10));
}
