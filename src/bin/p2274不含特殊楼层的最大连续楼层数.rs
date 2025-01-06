struct Solution;
impl Solution {
    pub fn max_consecutive(bottom: i32, top: i32, special: Vec<i32>) -> i32 {
        let mut special = special;
        special.push(bottom - 1);
        special.push(top + 1);
        special.sort_unstable();
        special.windows(2).map(|w| w[1] - w[0] - 1).max().unwrap()
    }
}
fn main() {}
