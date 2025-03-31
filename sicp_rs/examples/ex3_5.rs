use rand::Rng;

fn random_in_range(low: f64, high: f64) -> f64 {
    let mut rng = rand::rng();
    rng.random_range(low..high)
}
fn monte_carlo(trials: i32, experiment: impl Fn(i32) -> bool) -> f64 {
    // 递归实现迭代次数过多会导致栈溢出
    // fn iter(
    //     trials_remaining: i32,
    //     passed: i32,
    //     trials: i32,
    //     experiment: impl Fn(i32) -> bool,
    // ) -> f64 {
    //     if trials_remaining == 0 {
    //         return passed as f64 / trials as f64;
    //     }
    //     let passed = if experiment(trials_remaining) {
    //         passed + 1
    //     } else {
    //         passed
    //     };
    //     iter(trials_remaining - 1, passed, trials, experiment)
    // }
    // iter(trials, 0, trials, experiment)
    let mut passed = 0;
    for _ in 0..trials {
        if experiment(trials) {
            passed += 1;
        }
    }
    passed as f64 / trials as f64
}
fn estimate_integral(p: impl Fn(f64, f64) -> bool, x1: f64, x2: f64, y1: f64, y2: f64) -> f64 {
    let trials = 1000;
    monte_carlo(trials, |_| {
        let x = random_in_range(x1, x2);
        let y = random_in_range(y1, y2);
        p(x, y)
    })
}
fn main() {
    let p = |x: f64, y: f64| x * x + y * y <= 1.0;
    let result = estimate_integral(p, -1.0, 1.0, -1.0, 1.0);
    println!("{}", result * 4.0); // 3.140968
}
