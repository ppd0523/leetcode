struct Solution;
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a_v: Vec<char> = a.chars().rev().collect();
        let mut b_v: Vec<char> = b.chars().rev().collect();
        let mut r_v: Vec<char> = Vec::new();
        let mut carry = false;
        
        let max_len = a_v.len().max(b_v.len());
        a_v.resize(max_len, '0');
        b_v.resize(max_len, '0');
        r_v.resize(max_len, '0');
        
        for i in 0..max_len {
            match (a_v[i], b_v[i], carry) {
                ('1', '1', true) => {
                    r_v[i] = '1';
                    carry = true;
                }
                ('1', '0', true) | ('0', '1', true) | ('1', '1', false) => {
                    r_v[i] = '0';
                    carry = true;
                }
                ('1', '0', false) | ('0', '1', false) | ('0', '0', true) => {
                    r_v[i] = '1';
                    carry = false;
                }
                ('0', '0', false) => {
                    r_v[i] = '0';
                    carry = false;
                }
                _ => unreachable!(),
            }
        }
        if carry {
            r_v.push('1');
        }
        
        r_v.iter().rev().collect()
    }
}

fn main() {
    let a = "10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101".to_string();
    let b = "110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011".to_string();
    let res = Solution::add_binary(a, b);
    dbg!(res);
}