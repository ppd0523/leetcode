struct Solution;
impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        -1
    }
}

fn main() {
    let in1a = 3;
    let in1b = vec![vec![0, 1], vec![1, 2]];
    let out1 = Solution::find_champion(in1a, in1b);
    dbg!(out1);
}
