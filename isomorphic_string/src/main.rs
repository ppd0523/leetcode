use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut s_m: HashMap<char, u32> = HashMap::new();
        let mut s_v: Vec<u32> = Vec::new();
        let mut t_m: HashMap<char, u32> = HashMap::new();
        let mut t_v: Vec<u32> = Vec::new();
        
        let mut s_cnt = 0;
        s.chars().for_each(|c| {
            let e = s_m.entry(c).or_insert(s_cnt);
            s_cnt += 1;
            s_v.push(*e);
        });
        
        let mut t_cnt = 0;
        t.chars().for_each(|c| {
            let e = t_m.entry(c).or_insert(t_cnt);
            t_cnt += 1;
            t_v.push(*e);
        });
        
        s_v == t_v
    }
}

fn main() {
    let input1_a = dbg!("egg".to_string());
    let input1_b = dbg!("add".to_string());
    let res1 = Solution::is_isomorphic(input1_a, input1_b);
    dbg!(res1);

    let input2_a = dbg!("foo".to_string());
    let input2_b = dbg!("bar".to_string());
    let res2 = Solution::is_isomorphic(input2_a, input2_b);
    dbg!(res2);
    
    let input3_a = dbg!("paper".to_string());
    let input3_b = dbg!("title".to_string());
    let res3 = Solution::is_isomorphic(input3_a, input3_b);
    dbg!(res3);
}