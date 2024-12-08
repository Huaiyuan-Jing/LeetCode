use std::{collections::HashSet};
const INF: i32 = 0x3f3f3f3f;
struct Solution;
impl Solution {
    /// Given a board and a row index i, return the bit representation of the positions of all elements in the ith row.
    /// The jth bit is 1 if the element at the jth column is 1 and 0 if the element at the jth column is 0.
    fn get_row(board: &Vec<Vec<i32>>, i: usize) -> i32 {
        let mut ans = 0;
        for j in 0..board.len() {
            if board[i][j] == 0 {continue}
            ans += 1 << j;
        }
        ans
    }
    /// Given a board and a column index i, return the bit representation of the positions of all elements in the ith column.
    /// The jth bit is 1 if the element at the jth row is 1 and 0 if the element at the jth row is 0.
    fn get_col(board: &Vec<Vec<i32>>, i: usize) -> i32 {
        let mut ans = 0;
        for j in 0..board.len() {
            if board[j][i] == 0 {continue}
            ans += 1 << j;
        }
        ans
    }
    /// Return the minimum number of steps to transform `st` to `ed` on a chessboard.
    /// The steps are swaps of two elements in the same row or column.
    /// Return INF if it is impossible to transform `st` to `ed`.
    fn steps_to_target(st: i32, ed: i32) -> i32 {
        if st.count_ones() != ed.count_ones() {return INF} else {(st ^ ed).count_ones() as i32 >> 1} 
    }
    /// Given a board, return the minimum number of steps to transform the board into a chessboard.
    /// The steps are swaps of two elements in the same row or column.
    /// Return -1 if it is impossible to transform the board into a chessboard.
    pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let mut set = HashSet::new();
        for i in 0..n {
            set.insert(Solution::get_row(&board, i));
        }
        // println!("{:?}", set);
        if set.len() != 2 {return -1}
        let mut iter = set.iter();
        let (r1, r2) = (*iter.next().unwrap(), *iter.next().unwrap());
        set.clear();
        // println!("{} {}", r1, r2);
        for i in 0..n {
            set.insert(Solution::get_col(&board, i));
        }
        // println!("{:?}", set);
        if set.len() != 2 {return -1}
        let mut iter = set.iter();
        let (c1, c2) = (*iter.next().unwrap(), *iter.next().unwrap());
        // println!("{} {}", c1, c2);
        if (r1 ^ r2 != (1 << n) - 1) || (c1 ^ c2 != (1 << n) - 1) {return -1}
        let target = (0..n).step_by(2).fold(0, |acc, x| acc | (1 << x));
        let ans = Solution::steps_to_target(r1, target).min(Solution::steps_to_target(r2, target)) + Solution::steps_to_target(c1, target).min(Solution::steps_to_target(c2, target));
        if ans >= INF {return -1} else {return ans}
    }
}
fn main() {
    let board = vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![1, 0, 0, 1], vec![1, 0, 0, 1]];
    println!("{}", Solution::moves_to_chessboard(board));
}
