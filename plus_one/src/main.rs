struct Solution;
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut carry = 0;
        
        *digits.last_mut().unwrap() += 1;
        
        for digit in digits.iter_mut().rev() {
            *digit += carry;
            if *digit == 10 {
                *digit -= 10;
                carry = 1;
            } else {
                carry = 0;
                break;
            }
//            *digit = value % 10;
//            carry = value / 10;
//                        
//            if carry == 0 {
//                break;
//            }
        }
        if carry != 0 {
            digits.insert(0, 1);
        }
        
        digits
    }
}
        


fn main() {
//    let input1 = [1, 2, 3].to_vec();
//    let res1 = Solution::plus_one(dbg!(input1));
//    dbg!(res1);
//    
//    let input2 = [4, 3, 2, 1].to_vec();
//    let res2 = Solution::plus_one(dbg!(input2));
//    dbg!(res2);
//    
//    let input3 = [9].to_vec();
//    let res3 = Solution::plus_one(dbg!(input3));
//    dbg!(res3);
    
    let input4 = [9, 9].to_vec();
    let res4 = Solution::plus_one(dbg!(input4));
    dbg!(res4);
}
