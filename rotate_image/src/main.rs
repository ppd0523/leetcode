struct Solution;
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix.len();
        
        for r in 0..len {
            for c in r+1..len {
                let temp = matrix[r][c];
                matrix[r][c] = matrix[c][r];
                matrix[c][r] = temp;
            }
            matrix[r].reverse();
        }
    }
}

fn main() {
    let mut in1 = vec![
        vec![1,2,3],
        vec![4,5,6],
        vec![7,8,9],
    ];
    Solution::rotate(&mut in1);
    for rows in in1.iter_mut() {
        for col in rows.iter_mut() {
            print!("{col}");
        }
        println!("");
    }
}