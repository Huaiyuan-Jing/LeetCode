use std::collections::HashSet;
use std::mem;

impl Solution {
    pub fn max_value(nums: Vec<i32>, k: i32) -> i32 {
        let mut prefix_or: Vec<HashSet<i32>> = vec![HashSet::new(); k as usize + 1];
        let mut dp_left: Vec<HashSet<i32>> = vec![HashSet::new(); nums.len()];
        prefix_or[0].insert(0);
        for i in 0..nums.len() {
            for j in (1..=k).rev() {
                let previous_set = mem::take(&mut prefix_or[j as usize - 1]); // 将其取出
                previous_set.iter().for_each(|x| {
                    prefix_or[j as usize].insert(*x | nums[i]);
                });
                prefix_or[j as usize - 1] = previous_set;
            }
            dp_left[i] = prefix_or[k as usize].clone();
        }
        let mut dp_right: Vec<HashSet<i32>> = vec![HashSet::new(); nums.len()];
        prefix_or[0].insert(0);
        for i in (0..nums.len()).rev() {
            for j in (1..=k).rev() {
                let previous_set = mem::take(&mut prefix_or[j as usize - 1]); // 将其取出
                previous_set.iter().for_each(|x| {
                    prefix_or[j as usize].insert(*x | nums[i]);
                });
                prefix_or[j as usize - 1] = previous_set;
            }
            dp_right[i] = prefix_or[k as usize].clone();
        }
        let mut ans = 0;
        for i in k as usize - 1..nums.len() - 1 {
            
            for x in &dp_left[i] {
                for y in &dp_right[i + 1] {
                    ans = ans.max(x ^ y);
                }
            }
        }
        ans
    }
}
fn main() {}
struct Solution;
