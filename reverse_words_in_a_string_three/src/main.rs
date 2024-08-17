struct Solution;
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace()
            .into_iter()
            .map(|w| w.chars().rev().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

fn main() {
    let in1 = "Let's take LeetCode contest".to_string();
    let out1 = Solution::reverse_words(in1);
    dbg!(out1);
}
