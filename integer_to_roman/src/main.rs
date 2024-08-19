struct Solution;
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut ans = String::new();

        let mut rem = num;

        while rem > 0 {
            if rem >= 1000 {
                rem -= 1000;
                ans.push('M');
            } else if rem >= 900 {
                rem -= 900;
                ans.push_str("CM");
            } else if rem >= 500 {
                rem -= 500;
                ans.push('D');
            } else if rem >= 400 {
                rem -= 400;
                ans.push_str("CD");
            } else if rem >= 100 {
                rem -= 100;
                ans.push('C');
            } else if rem >= 90 {
                rem -= 90;
                ans.push_str("XC");
            } else if rem >= 50 {
                rem -= 50;
                ans.push('L');
            } else if rem >= 40 {
                rem -= 40;
                ans.push_str("XL");
            } else if rem >= 10 {
                rem -= 10;
                ans.push('X');
            } else if rem >= 9 {
                rem -= 9;
                ans.push_str("IX");
            } else if rem >= 5 {
                rem -= 5;
                ans.push('V');
            } else if rem >= 4 {
                rem -= 4;
                ans.push_str("IV");
            } else if rem >= 1 {
                rem -= 1;
                ans.push('I');
            }
        }

        ans
    }
}

fn main() {
    let in1 = 3749;
    let out1 = Solution::int_to_roman(in1);
    dbg!(out1);

    let in2 = 58;
    let out2 = Solution::int_to_roman(in2);
    dbg!(out2);
}
