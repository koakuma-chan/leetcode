/// You are given a large integer represented as an integer array digits, where
/// each digits[i] is the ith digit of the integer. The digits are ordered from
/// most significant to least significant in left-to-right order. The large
/// integer does not contain any leading 0's.
///
/// Increment the large integer by one and return the resulting array of digits.

struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for i in (0..digits.len()).rev() {
            let digit = &mut digits[i];
            let plus_one = *digit + 1;
            if plus_one == 10 {
                *digit = 0;
            } else {
                *digit = plus_one;
                return digits;
            }
        }
        digits.insert(0, 1);
        digits
    }
}

pub fn main() {
    println!("{:?}", Solution::plus_one(vec![9, 9, 9]));
}
