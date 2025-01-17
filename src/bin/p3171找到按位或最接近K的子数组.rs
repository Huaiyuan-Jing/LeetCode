struct Solution;
impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        let (mut ans, mut b, mut l, mut right_or) = (i32::MAX, 0, 0, 0);
        for r in 0..nums.len() {
            right_or |= nums[r];
            while l <= r && (right_or | nums[l]) >= k {
                ans = ans.min(((right_or | nums[l]) - k).abs());
                l += 1;
                if b < l {
                    for i in (l..r).rev() {
                        nums[i] |= nums[i + 1];
                    }
                    b = r;
                    right_or = 0;
                }
            }
            if l < nums.len() {
                ans = ans.min(((right_or | nums[l]) - k).abs());
            }
        }
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}
fn main() {}
