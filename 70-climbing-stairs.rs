/// You are climbing a staircase. It takes n steps to reach the top.
///
/// Each time you can either climb 1 or 2 steps. In how many distinct ways can
/// you climb to the top?

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 1 {
            return 1;
        }
        let mut a = 1;
        let mut b = 1;
        for _ in 2..n + 1 {
            let temp = a + b;
            a = b;
            b = temp;
        }
        return b;
    }
}

pub fn main() {
    println!("{}", Solution::climb_stairs(3));
}
