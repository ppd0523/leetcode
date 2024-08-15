use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut m: HashMap<i32, i32> = HashMap::new();
        let size = nums.len();
        
        nums.into_iter().for_each(|n| {
            let c = m.entry(n)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        });
        
        let (k, v) = m.iter().max_by_key(|e| e.1).unwrap();
        *k
    }
}

fn main() {
    let input1 = vec![3, 2, 3];
    let res1 = Solution::majority_element(input1);
    dbg!(res1);
    
    let input2 = vec![2, 2, 1, 1, 1, 2, 2];
    let res2 = Solution::majority_element(input2);
    dbg!(res2);
    
    let input3 = vec![3, 3, 4];
    let res3 = Solution::majority_element(input3);
    dbg!(res3);
}