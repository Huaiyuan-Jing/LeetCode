struct Solution;
impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![vec![i64::MAX; k as usize + 1]; nums.len()];
        let prefix = nums
            .iter()
            .scan(0, |state, x| {
                *state += x;
                Some(*state as i64)
            })
            .collect::<Vec<_>>();
        (0..nums.len()).for_each(|i| dp[i][1] = prefix[i]);
        for j in 2..=k as usize {
            for i in 0..nums.len() {
                if j > i + 1 {
                    continue;
                }
                dp[i][j] = (0..nums.len() - 1)
                    .map(|l| dp[l][j - 1].max(prefix[i] - prefix[l]))
                    .min()
                    .unwrap();
            }
        }
        // for j in 1..=k as usize {
        //     for i in 0..nums.len() {
        //         print!("{} ", dp[i][j]);
        //     }
        //     println!();
        // }
        dp[nums.len() - 1][k as usize] as i32
    }
}
fn main() {
    let nums = vec![1, 4, 4];
    let k = 3;
    println!("{}", Solution::split_array(nums, k));
}
