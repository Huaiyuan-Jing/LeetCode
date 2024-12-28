struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut ptr = 0 as usize;
        for i in 0..nums.len() {
            if nums[i] == nums[ptr] {
                continue;
            }
            ptr += 1;
            nums[ptr] = nums[i];
        }
        ptr as i32 + 1
    }
}
fn main() {}
