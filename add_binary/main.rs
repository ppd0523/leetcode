fn convert_int(s: &String) -> i64 {
    let v: Vec<char> = s.chars().rev().collect();
    let mut total = 0_i64;
        
    for (i, &c) in v.iter().enumerate() {
        if c == '1' {
            total += 2_i64.pow(i as u32);
        }
    }
    total
}

struct Solution;
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let left = convert_int(&a);
        let right = convert_int(&b);
        
        format!("{:b}", left+right)
    }
}

fn main() {
    let a = "10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101".to_string();
    let b = "110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011".to_string();
    let res = Solution::add_binary(a, b);
    dbg!(res);
}