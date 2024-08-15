struct Solution;
impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let mut v = Vec::<u8>::new();
        
        for &b in s.as_bytes() {
            match b {
                b'#' => {
                    let n2 = v.pop().unwrap();
                    let n1 = v.pop().unwrap();
                    let n = (n1-48) * 10 + (n2-48);
                    let n = n + 48;
                    v.push(n);
                },
                b =>{
                    v.push(b);
                },
            }
        }

        v.into_iter()
            .map(|n| (n+48) as char)
            .collect::<String>()
    }
}

fn main() {
    let in1 = "10#11#12".to_string();
    let out1 = Solution::freq_alphabets(in1);
    dbg!(out1);
}
