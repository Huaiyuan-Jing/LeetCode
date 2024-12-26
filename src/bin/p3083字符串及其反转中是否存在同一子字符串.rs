use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn is_substring_present(s: String) -> bool {
        let mut set = HashSet::new();
        for i in 0..s.len() - 1 {
            // println!("{}", &s[i..i + 2]);
            set.insert(s[i..i + 2].to_string());
        }
        for i in (1..s.len()).rev() {
            let mut tmp = String::new();
            tmp.push(s.chars().nth(i).unwrap());
            tmp.push(s.chars().nth(i - 1).unwrap());
            if set.contains(&tmp) {
                return true;
            }
        }
        false
    }
}
fn main() {}
