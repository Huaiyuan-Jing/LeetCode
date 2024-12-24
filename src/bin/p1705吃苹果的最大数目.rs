use std::cmp::Reverse;

struct Solution;
impl Solution {
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        let mut min_heap = std::collections::BinaryHeap::new();
        let mut ans = 0;
        for i in 0..apples.iter().sum::<i32>() as usize + apples.len() {
            if i < apples.len() {
                if apples[i] > 0 {
                    min_heap.push((-(days[i] + i as i32 - 1), apples[i]));
                }
            }
            while let Some((x, y)) = min_heap.pop() {
                if -x < i as i32 {
                    continue;
                }
                ans += 1;
                if y > 1 {
                    min_heap.push((x, y - 1));
                }
                break;
            }
            if i >= days.len() && min_heap.is_empty() {
                break;
            }
        }
        ans
    }
}
fn main() {}
