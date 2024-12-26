struct Solution;
impl Solution {
    pub fn mul(a: i64, b: i64) -> i64 {
        let mut a = a;
        let mut b = b;
        let mut res = 0 as i64;
        let mut neg_offset = 1;
        if a < 0 {
            neg_offset *= -1;
        }
        if b < 0 {
            neg_offset *= -1;
        }
        while b != 0 {
            if b & 1 == 1 {
                res += a;
            }
            b >>= 1;
            a <<= 1;
        }
        res * neg_offset
    }
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let mut neg_offset = 1;
        let dividend = dividend as i64;
        let divisor = divisor as i64;
        if dividend < 0 {
            neg_offset *= -1;
        }
        if divisor < 0 {
            neg_offset *= -1;
        }
        let dividend = dividend.abs();
        let divisor = divisor.abs();
        let mut l = 0 as i64;
        let mut r = dividend as i64;
        while l < r {
            let mid = (l + r + 1) / 2;
            if Solution::mul(mid, divisor) <= dividend {
                l = mid;
            } else {
                r = mid - 1;
            }
        }
        println!("{}", l);
        (l * neg_offset).min(2147483647).max(-2147483648) as i32
    }
}
fn main() {}
