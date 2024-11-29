struct Solution;

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let n = colors.len();
        let mut seq = vec![];
        let mut cnt = 1;
        for i in 0..n {
            if i == n - 1 {
                seq.push(cnt);
                continue;
            }
            if colors[i] == colors[i + 1] {
                seq.push(cnt);
                cnt = 1;
                continue;
            }
            cnt += 1;
        }
        if seq.len() == 1 && colors[0] != colors[n - 1] {
            return if seq[0] < k {0} else {seq[0]};
        }
        if seq.len() > 1 && colors[0] != colors[n - 1] {
            seq[0] += seq[seq.len() - 1];
            seq.pop();
        }
        seq.iter().map(|x| (x - k + 1).max(0)).sum()
    }
}

fn main() {
    let colors = vec![0, 1, 0, 1];
    let k = 3;
    println!("{}", Solution::number_of_alternating_groups(colors, k));
}
