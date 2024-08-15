struct Solution;
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        let mut cnt = 0;
        if n == 0 {
            return false;
        }
        for i in 0..32 {
            if (n >> i) & 0x00_00_00_01 == 0x01 {
                cnt += 1;
            }
        }
        
        if cnt > 1 {
            return false;
        } else {
            return true;
        }
    }
}

fn main () {
    let input1 = 8;
    dbg!(Solution::is_power_of_two(dbg!(input1)));
    let input2 = 16;
    dbg!(Solution::is_power_of_two(dbg!(input2)));
    let input3 = 3;
    dbg!(Solution::is_power_of_two(dbg!(input3)));
    let input4 = -16;
    dbg!(Solution::is_power_of_two(dbg!(input4)));
    let input5 = -2147483648;
    dbg!(Solution::is_power_of_two(dbg!(input5)));
}