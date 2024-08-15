struct Solution;
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut change5: i32 = 0;
        let mut change10: i32 = 0;

        for &bill in bills.iter() {
            match bill {
                5 => {
                    change5 += 1
                },
                10 => {
                    change10 += 1;
                    if change5 >= 1 {
                        change5 -=1;
                    } else {
                        return false;
                    }
                },
                20 => {
                    if change10 >= 1 && change5 >= 1 {
                        change10 -= 1;
                        change5 -= 1;
                    } else if change5 >= 3 {
                        change5 -= 3;
                    } else {
                        return false;
                    }
                },
                _ => {},
            }
        };

        true
    }
}

fn main() {
    let in1 = vec![5, 5, 5, 10, 20];
    let out1 = Solution::lemonade_change(in1);
    dbg!(out1);

    let in2 = vec![5,5,10,10,20];
    let out2 = Solution::lemonade_change(in2);
    dbg!(out2);

    let in3 = vec![5,5,5,10,5,5,10,20,20,20];
    let out3 = Solution::lemonade_change(in3);
    dbg!(out3);
}