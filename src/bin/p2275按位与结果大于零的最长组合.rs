struct Solution;
impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut candidates = candidates;
        while *candidates.iter().max().unwrap() > 0 {
            ans = ans.max(candidates.iter().map(|&x| x & 1).sum());
            candidates = candidates.iter().map(|&x| x >> 1).collect();
        }
        ans
    }
}
fn main() {
    let candidates = vec![16, 17, 71, 62, 12, 24, 14];
    println!("{}", Solution::largest_combination(candidates));
}
