use core::num;

struct Solution;
impl Solution {
    pub fn generate_key(num1: i32, num2: i32, num3: i32) -> i32 {
        let mut num1 = num1;
        let mut num2 = num2;
        let mut num3 = num3;
        let mut ans = 0;
        let mut pos = 1;
        for _ in 0..4 {
            let digit = (num1 % 10).min(num2 % 10).min(num3 % 10);
            ans += digit * pos;
            pos *= 10;
            num1 /= 10;
            num2 /= 10;
            num3 /= 10;
        }
        ans
    }
}
fn main() {}
