fn main() {
    let factorial = |n: i32| {
        let mut res = 1;
        for i in 1..=n {
            res *= i;
        }
        res
    };
    let bion = |n: i32, p: f32, k: i32| {
        let res = factorial(n) / (factorial(k) * factorial(n - k));
        res as f32 * p.powf(k as f32) * (1.0 - p).powf((n - k) as f32)
    };
    println!("{}", bion(5, 0.01, 3) / (bion(5, 0.01, 3) + bion(5, 0.01, 4) + bion(5, 0.01, 5)));
}
