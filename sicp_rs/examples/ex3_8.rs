fn make_f() -> impl FnMut(i32) -> i32 {
    let mut x = 1;
    // FnMut 的调用特性只表示闭包可以修改自身的状态，但它并不限定闭包如何捕获外部变量，并不一定是按可变引用捕获。
    let closure = move |y: i32| {
        x *= y;
        x
    };
    closure
}
fn main() {
    let mut f = make_f();
    println!("rust: f(0) + f(1) = {}", f(0) + f(1));

    let mut f = make_f();
    let a = f(0);
    let b = f(1);
    println!("from left to right: f(0) + f(1) = {}", a + b);
    let mut f = make_f();
    let a = f(1);
    let b = f(0);
    println!("from right to left: f(0) + f(1) = {}", a + b);
}
