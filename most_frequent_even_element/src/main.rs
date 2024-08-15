use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let mut m = HashMap::new();
        let mut max_v = i32::MIN;
        let mut min_k = i32::MAX;
        nums.into_iter().for_each(|n| {
            if n % 2 == 0 {
                let e = m.entry(n)
                    .and_modify(|v| *v += 1)
                    .or_insert(1);

                max_v = max_v.max(*e);
            }
        });

        if m.is_empty() {
            return -1;
        }

        for (k, v) in m.into_iter() {
            if v == max_v {
                min_k = min_k.min(k);
            }
        }

        min_k
    }
}

fn main() {
    let input1 = vec![0,1,2,2,4,4,1];
    let res1 = Solution::most_frequent_even(input1);
    dbg!(res1);
    
    // let input2 = vec![4,4,4,9,2,4];
    // let res2 = Solution::most_frequent_even(input2);
    // dbg!(res2);
    //
    // let input3 = vec![29,47,21,41,13,37,25,7];
    // let res3 = Solution::most_frequent_even(input3);
    // dbg!(res3);
}