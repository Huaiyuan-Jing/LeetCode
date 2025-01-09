struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            return Solution::find_median_sorted_arrays(nums2, nums1);
        }
        let k = (nums1.len() + nums2.len() + 1) / 2;
        let mut l = 0;
        let mut r = nums1.len();
        while l < r {
            // println!("{} {}", l, r);
            let p1 = l + (r - l) / 2;
            let p2 = k - p1;
            if nums1[p1] < nums2[p2 - 1] {
                l = p1 + 1;
            } else {
                r = p1;
            }
        }
        let p1 = l;
        let p2 = k - l;
        let c1 = (if p1 <= 0 {
            std::i32::MIN
        } else {
            nums1[p1 - 1]
        })
        .max(if p2 <= 0 {
            std::i32::MIN
        } else {
            nums2[p2 - 1]
        });
        if (nums1.len() + nums2.len()) % 2 == 1 {
            return c1 as f64;
        } 
        let c2 = (if p1 >= nums1.len() {
            std::i32::MAX
        } else {
            nums1[p1]
        })
        .min(if p2 >= nums2.len() {
            std::i32::MAX
        } else {
            nums2[p2]
        });
        return (c1 + c2) as f64 / 2.0;
    }
}
fn main() {
    println!(
        "{}",
        Solution::find_median_sorted_arrays(vec![1, 3], vec![2])
    );
}
