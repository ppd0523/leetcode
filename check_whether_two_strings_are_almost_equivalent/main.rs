use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        let mut c1: HashMap<char, u32> = HashMap::new();
        let mut c2: HashMap<char, u32> = HashMap::new();
        
        word1.chars().for_each(|c| {
            let v = c1.entry(c).or_insert(0);
            *v = *v + 1;
        });
        
        word2.chars().for_each(|c| {
            let v = c2.entry(c).or_insert(0);
            *v = *v + 1;
        });
        
        let mut c1_key: Vec<char> = dbg!(c1.keys().cloned().collect());
        let mut c2_key: Vec<char> = dbg!(c2.keys().cloned().collect());
        
        c1_key.append(&mut c2_key);
        c1_key = dbg!(c1_key);

        let mut diff = 0_u32;
        for key in &c1_key {
            let lhs = c1.get(key).unwrap_or(&0);
            let rhs = c2.get(key).unwrap_or(&0);
            
            let _diff =  lhs.abs_diff(*rhs);
            diff = _diff.max(diff);
        }
        
        if diff > 3 {
            false
        } else {
            true
        }
    }
}

fn main() {
    let input1_a = "aaaa";
    let input1_b = "bccb";
    let res1 = Solution::check_almost_equivalent(input1_a.to_string(), input1_b.to_string());
    dbg!(res1);
    
    let input2_a = "abcdeef";
    let input2_b = "abaaacc";
    let res2 = Solution::check_almost_equivalent(input2_a.to_string(), input2_b.to_string());
    dbg!(res2);
}