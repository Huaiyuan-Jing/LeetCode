struct Solution;
impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let mut prefix_sums: Vec<_> = nums
            .iter()
            .scan(0i64, |state, x| {
                *state += *x as i64;
                Some(*state as i64)
            })
            .collect();
        let sum = prefix_sums.pop().unwrap();
        prefix_sums.into_iter().filter(|x| *x >= sum - *x).count() as i32
    }
}
fn main() {
    println!(
        "{}", 
        Solution::ways_to_split_array(vec![10, 4, 8, 3]))
}
