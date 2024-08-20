struct Solution;
impl Solution {
    pub fn last_non_empty_string(s: String) -> String {
        let mut dict = [0_i32; 256];
        let mut max_cnt = 0;

        for c in s.chars() {
            dict[c as usize] += 1;
        }

        max_cnt = *dict.iter().max().unwrap();

        let mut _ans = s
            .chars()
            .rev()
            .filter(|c| {
                if dict[*c as usize] == max_cnt {
                    dict[*c as usize] = 0;
                    return true;
                } else {
                    return false;
                }
            })
            .collect::<String>();

        _ans.chars().rev().collect()
    }
}

fn main() {
    let in1 = "aabcbbca".to_string();
    let out1 = Solution::last_non_empty_string(in1);
    dbg!(out1);

    let in2 = "abcd".to_string();
    let out2 = Solution::last_non_empty_string(in2);
    dbg!(out2);
}
