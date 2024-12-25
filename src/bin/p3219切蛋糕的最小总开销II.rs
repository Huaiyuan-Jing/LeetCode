struct Solution;
impl Solution {
    pub fn minimum_cost(m: i32, n: i32, horizontal_cut: Vec<i32>, vertical_cut: Vec<i32>) -> i64 {
        let m = m as usize;
        let n = n as usize;
        let mut horizontal_cut = horizontal_cut;
        let mut vertical_cut = vertical_cut;
        horizontal_cut.sort_unstable_by(|a, b| b.cmp(a));
        vertical_cut.sort_unstable_by(|a, b| b.cmp(a));
        let mut ans = 0 as i64;
        let mut horizontal_coefficient = 1;
        let mut vertical_coefficient = 1;
        let mut p_horizontal = 0;
        let mut p_vertical = 0;
        for _ in 0..(n - 1 + m - 1) {
            if p_horizontal >= m - 1 {
                ans += vertical_coefficient as i64 * vertical_cut[p_vertical] as i64;
                p_vertical += 1;
                horizontal_coefficient += 1;
                continue;
            }
            if p_vertical >= n - 1 {
                ans += horizontal_coefficient as i64 * horizontal_cut[p_horizontal] as i64;
                p_horizontal += 1;
                vertical_coefficient += 1;
                continue;
            }
            if horizontal_cut[p_horizontal] > vertical_cut[p_vertical] {
                ans += horizontal_coefficient as i64 * horizontal_cut[p_horizontal] as i64;
                p_horizontal += 1;
                vertical_coefficient += 1;
            } else {
                ans += vertical_coefficient as i64 * vertical_cut[p_vertical] as i64;
                p_vertical += 1;
                horizontal_coefficient += 1;
            }
        }
        ans
    }
}
fn main() {}
