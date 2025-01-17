struct Solution;
impl Solution {
    fn check(prefix: &Vec<i32>, window: i32, k: i32) -> bool {
        for i in 0..prefix.len() {
            if i + window as usize - 1 >= prefix.len() {
                return false;
            }
            if 
        }
        false
    }
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut l = 1;
        let mut r = nums.len();
        while l < r {
            let mid = (l + r) / 2;
            if Solution::check(&prefix_or, mid as i32, k) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        if Solution::check(&prefix_or, r as i32, k) {
            r as i32
        } else {
            -1
        }
    }
}
fn main() {}
