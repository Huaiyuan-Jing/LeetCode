struct Solution;
impl Solution {
    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        let mut diff: Vec<i64> = vec![0; 26];
        let n = word1.len();
        let mut ans: i64 = 0;
        for ch in word2.bytes() {
            diff[ch as usize - 'a' as usize] -= 1;
        }
        let mut cnt = diff.iter().filter(|val| **val < 0).count();
        let mut update_cnt = |ch: u8, add: i8, cnt: &mut usize| {
            let ch = ch as usize - 'a' as usize;
            diff[ch] += add as i64;
            if add == 1 && diff[ch] == 0 {
                *cnt -= 1;
            } else if add == -1 && diff[ch] == -1 {
                *cnt += 1;
            }
        };
        let mut l = 0;
        let mut r = 0;
        while r < n {
            update_cnt(word1.bytes().nth(r).unwrap(), 1, &mut cnt);
            r += 1;
            while cnt == 0 {
                ans += (n - r + 1) as i64;
                update_cnt(word1.bytes().nth(l).unwrap(), -1, &mut cnt);
                l += 1;
            }
        }
        ans
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::valid_substring_count("bcca".to_string(), "abc".to_string())
    );
}
