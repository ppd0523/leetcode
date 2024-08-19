struct Solution;
impl Solution {
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        let mut dict = dictionary.clone();
        dict.sort_by(|a, b| {
            let al = a.len();
            let bl = b.len();

            if al == bl {
                a.cmp(b)
            } else {
                bl.cmp(&al)
            }
        });
        dict = dbg!(dict);
        for word in dict {
            let mut sc = 0;
            let mut wc = 0;

            while sc < s.len() {
                if &s.chars().nth(sc) == &word.chars().nth(wc) {
                    wc += 1;
                }
                sc += 1;
            }
            if wc == word.len() {
                return word;
            }
        }

        "".to_string()
    }
}

fn main() {
    let in1a = "abpcplea".to_string();
    let in1b = vec![
        "ale".to_string(),
        "apple".to_string(),
        "monkey".to_string(),
        "plea".to_string(),
    ];
    let out1 = Solution::find_longest_word(in1a, in1b);
    dbg!(out1);

    let in2a = "abpcplea".to_string();
    let in2b = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let out2 = Solution::find_longest_word(in2a, in2b);
    dbg!(out2);

    let in3a = "abce".to_string();
    let in3b = vec!["abe".to_string(), "abc".to_string()];
    let out3 = Solution::find_longest_word(in3a, in3b);
    dbg!(out3);
}
