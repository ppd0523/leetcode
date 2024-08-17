struct Solution;
impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let mut v = Vec::<i32>::with_capacity(4);

        


        v.into_iter().max_by_key(|k| *k).unwrap()
    }
}

fn main() {
    let in1= vec![
        vec![1,2],
        vec![2,3],
        vec![4,2],
    ];
    let out1 = Solution::find_center(in1);
    dbg!(out1);

}