struct Solution;
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m - 1;
        let mut j = n - 1;
        for cur in (0..(n+m) as usize).rev() {
            if i < 0 {
                nums1[cur] = nums2[j as usize];
                j -= 1;
                continue;
            }
            if j < 0 {
                nums1[cur] = nums1[i as usize];
                i -= 1;
                continue;
            }
            if nums1[i as usize] > nums2[j as usize] {
                nums1[cur] = nums1[i as usize];
                i -= 1;
            }
            else {
                nums1[cur] = nums2[j as usize];
                j -= 1;
            }
        }
    }
}
fn main () {}
