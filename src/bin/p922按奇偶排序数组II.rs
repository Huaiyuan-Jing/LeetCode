use std::mem::swap;

impl Solution {
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable_by_key(|&x| x % 2);
        let mut p1 = 1;
        let mut p2 = if (nums.len() - 1) % 2 == 0 {
            nums.len() - 1
        } else {
            nums.len() - 2
        };
        while p1 < p2 {
            nums.swap(p1, p2);
            p1 += 2;
            p2 -= 2;
        }
        nums
    }
}
struct Solution;

fn main() {}
