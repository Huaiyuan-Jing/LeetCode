impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut res = vec![1];
        for i in 1..=row_index {
            for j in (1..i as usize).rev() {
                res[j] += res[j-1];
            }
            res.push(1);
        }
        res    
    }
}
struct Solution;

fn main() {}
