struct Solution;
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let n = prices.len();
        let k = k.min(n as i32 / 2) as usize;
        let mut buy = vec![vec![-0x3f3f3f3f; k + 1]; n];
        let mut sell = vec![vec![-0x3f3f3f3f; k + 1]; n];
        buy[0][0] = -prices[0];
        sell[0][0] = 0;
        for i in 1..n {
            sell[i][0] = 0;
            buy[i][0] = buy[i - 1][0].max(sell[i - 1][0] - prices[i]);
            for j in 1..=k {
                buy[i][j] = buy[i - 1][j].max(sell[i - 1][j] - prices[i]);
                sell[i][j] = sell[i - 1][j].max(buy[i - 1][j - 1] + prices[i]);
            }
        }
        *sell[n - 1].iter().max().unwrap()
    }
}
fn main() {
    let k = 2;
    let prices = vec![2, 4, 1];
    println!("{}", Solution::max_profit(k, prices));
}
