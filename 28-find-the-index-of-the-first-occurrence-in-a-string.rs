/// Given two strings needle and haystack, return the index of the first
/// occurrence of needle in haystack, or -1 if needle is not part of haystack.

struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        use std::convert::TryInto;
        let mut i = 0;
        while let Some(substring) = haystack.get(i..i + needle.len()) {
            if substring == needle {
                return i.try_into().unwrap();
            }
            i += 1;
        }
        -1
    }
}

pub fn main() {
    println!(
        "{}",
        Solution::str_str("leetcode".to_string(), "code".to_string())
    );
}
