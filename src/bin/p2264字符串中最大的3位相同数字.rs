struct Solution;
impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        num.chars()
            .collect::<Vec<_>>()
            .windows(3)
            .filter(|window| window[0] == window[1] && window[1] == window[2])
            .map(|window| window.iter().collect::<String>())
            .max()
            .unwrap_or(String::new())
    }
}
fn main() {}
