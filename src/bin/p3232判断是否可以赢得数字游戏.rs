struct Solution;
impl Solution {
    pub fn can_alice_win(nums: Vec<i32>) -> bool {
        nums.iter().map(|&x| if x < 10 { x } else { -x }).sum::<i32>() != 0
    }
}
fn main() {
    let nums = vec![1, 2, 3, 4, 10];
    println!("{}", Solution::can_alice_win(nums));
}
