struct Solution;
impl Solution {
    pub fn interpret(command: String) -> String {
        let mut prev = ' ';
        let mut ans = String::new();

        for c in command.chars() {
            match (prev, c) {
                (_, 'G') => ans.push('G'),
                ('(', ')') => ans.push('o'),
                ('l', ')') => ans.push_str("al"),
                (_, c) => prev = c,
            };
        }

        ans
    }
}

fn main() {
    let in1 = "G()(al)".to_string();
    let out1 = Solution::interpret(in1);
    dbg!(out1);
}
