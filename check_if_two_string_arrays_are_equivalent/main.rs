struct Solution;
impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1.concat() == word2.concat()
    }
}

fn main() {
    let input1_a = vec!["ab".to_string(), "c".to_string()];
    let input1_b = vec!["a".to_string(), "bc".to_string()];
    dbg!(Solution::array_strings_are_equal(input1_a, input1_b));
    
    let input2_a = vec!["a".to_string(), "cb".to_string()];
    let input2_b = vec!["ab".to_string(), "c".to_string()];
    dbg!(Solution::array_strings_are_equal(input2_a, input2_b));
}