use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        let mut word2_cnt = HashMap::new();
        for c in word2.chars() {
            *word2_cnt.entry(c).or_insert(0) += 1;
        }
        let mut ed = -1 as i32;
        let mut word1_cnt = HashMap::new();
        let mut cnt_char = 0;
        let mut ans = 0;
        for (_, c) in word1.chars().enumerate() {
            while ed < word1.len() as i32 && cnt_char < word2_cnt.len() {
                ed += 1;
                let c2 = word1.chars().nth(ed as usize).unwrap();
                *word1_cnt.entry(c2).or_insert(0) += 1;
                if word1_cnt[&c2] == word2_cnt[&c2] {
                    cnt_char += 1;
                }
            }
            ans += word1_cnt.len() as i64 - cnt_char as i64;
            *word1_cnt.entry(c).or_insert(0) -= 1;
            if word1_cnt[&c] == word2_cnt[&c] - 1 {
                cnt_char -= 1;
            }
        }
        ans
    }
}
fn main() {
    let word1 = "bbbb".to_string();
    let word2 = "b".to_string();
    println!("{}", Solution::valid_substring_count(word1, word2));
}
