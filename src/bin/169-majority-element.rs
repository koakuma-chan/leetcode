/// Given an array nums of size n, return the majority element.
///
/// The majority element is the element that appears more than ⌊n / 2⌋ times.
/// You may assume that the majority element always exists in the array.
///
/// Example 1:
///
/// Input: nums = [3,2,3]
/// Output: 3
///
/// Example 2:
///
/// Input: nums = [2,2,1,1,1,2,2]
/// Output: 2

fn main() {
    let nums = vec![2, 2, 1, 1, 1, 2, 2];

    assert_eq!(2, Solution::majority_element(nums));
}

struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut candidate = None;

        let mut count = 0;

        for num in nums {
            if candidate == Some(num) {
                count += 1;
            } else {
                if count == 0 {
                    candidate = Some(num);

                    count += 1;
                } else {
                    count -= 1;
                }
            }
        }

        candidate.unwrap()
    }
}
