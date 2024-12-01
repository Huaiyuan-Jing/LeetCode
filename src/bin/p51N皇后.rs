struct Solution;

impl Solution {
    pub fn dfs(
        cur: i32,
        left: i32,
        right: i32,
        pos: &mut Vec<usize>,
        n: usize,
    ) -> Vec<Vec<String>> {
        if pos.len() == n {
            let mut ans: Vec<String> = vec![];
            pos.iter().for_each(|&x| {
                let mut s = ".".repeat(n);
                s.replace_range(x..x + 1, "Q");
                ans.push(s);
            });
            return vec![ans];
        }
        let used = cur | left | right;
        let mut ans = vec![];
        for i in 0..n {
            if (used & (1 << i)) != 0 {
                continue;
            }
            pos.push(i);
            Solution::dfs(
                cur | (1 << i),
                (left | (1 << i)) << 1,
                (right | (1 << i)) >> 1,
                pos,
                n,
            )
            .iter()
            .for_each(|x| ans.push(x.clone()));
            pos.pop();
        }
        ans
    }
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        Solution::dfs(0, 0, 0, &mut vec![], n as usize)
    }
}
fn main() {
    println!("{:?}", Solution::solve_n_queens(9));
}
