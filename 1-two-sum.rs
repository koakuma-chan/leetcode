/// Given an array of integers nums and an integer target, return indices of the
/// two numbers such that they add up to target.
///
/// You may assume that each input would have exactly one solution, and you may
/// not use the same element twice.
///
/// You can return the answer in any order.

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        use std::convert::TryInto;

        let mut index = HashMap::<i32, usize>::with_capacity(nums.len());

        for i in 0..nums.len() {
            let num = nums[i];

            let diff = target - num;

            if let Some(k) = index.get(&diff).copied() {
                return vec![i.try_into().unwrap(), k.try_into().unwrap()];
            } else {
                index.insert(num, i);
            }
        }

        unreachable!();
    }
}

fn main() {
    println!("{:?}", Solution::two_sum(vec![3, 2, 4], 6));
}
