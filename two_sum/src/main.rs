struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans = vec![];
        let len = nums.len();

        for i in 0..len {
            let first = nums.get(i).unwrap();
            let value = target - *first;

            for j in i + 1..len {
                if *nums.get(j).unwrap() == value {
                    ans.push(i as i32);
                    ans.push(j as i32);
                }
            }
        }

        ans
    }
}

fn main() {
    let in1a = vec![2, 7, 11, 15];
    let in1b = 9;
    let out1 = Solution::two_sum(in1a, in1b);
    dbg!(out1);

    let in2a = vec![3, 2, 4];
    let in2b = 6;
    let out2 = Solution::two_sum(in2a, in2b);
    dbg!(out2);

    let in3a = vec![3, 3];
    let in3b = 6;
    let out3 = Solution::two_sum(in3a, in3b);
    dbg!(out3);
}
