use std::collections::hash_set::HashSet;

struct Solution;
impl Solution {
    pub fn rank_teams(mut votes: Vec<String>) -> String {
        let mut tickets = vec![vec![0; 26]; 26];
        for vote in &votes {
            for (i, c) in vote.chars().enumerate() {
                tickets[(c as u8 - b'A' as u8) as usize][i] -= 1;
            }
        }
        let ans = unsafe {votes[0].as_bytes_mut()};
        ans.sort_unstable_by_key(|&c| (&tickets[(c - b'A') as usize], c));
        votes[0].clone()
    }
}
fn main() {}
