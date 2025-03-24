/// Given a non-negative integer x, return the square root of x rounded down to
/// the nearest integer. The returned integer should be non-negative as well.
///
/// You must not use any built-in exponent function or operator.
///
///     For example, do not use pow(x, 0.5) in c++ or x ** 0.5 in python.

struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        use std::convert::TryInto;
        if x == 0 || x == 1 {
            return x;
        }
        let x: i64 = x.into();
        let mut range = 0..x;
        loop {
            let guess = (range.start + range.end) / 2;
            if guess * guess > x {
                range = range.start..guess;
            } else {
                range = guess..range.end;
            }
            if range.end - range.start == 1 {
                return range.start.try_into().unwrap();
            }
        }
    }
}

pub fn main() {
    println!("{}", Solution::my_sqrt(17));
}
