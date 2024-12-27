struct Solution;
impl Solution {
    pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
        if nums.len() & 1 == 1 {return false}
        let mut nums = nums;
        let mut cnt = vec![0;101];
        nums.sort();
        for i in 0..nums.len() {
            cnt[nums[i] as usize] += 1;
            if cnt[nums[i] as usize] > 2 {return false}
        }
        true
    }
}
fn main() {}
