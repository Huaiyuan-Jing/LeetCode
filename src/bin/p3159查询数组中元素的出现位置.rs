struct Solution;
impl Solution {
    pub fn occurrences_of_element(nums: Vec<i32>, queries: Vec<i32>, x: i32) -> Vec<i32> {
        let mut record = vec![];
        for i in 0..nums.len() {
            if nums[i] != x { continue; }
            record.push(i as i32);
        }
        let mut res = vec![];
        for i in 0..queries.len() {
            if queries[i] as usize > record.len() {
                res.push(-1);
                continue;
            }
            res.push(record[queries[i] as usize - 1]);
        }
        res
    }
}
fn main() {}
