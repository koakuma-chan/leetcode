/// Write a function to find the longest common prefix string amongst an array
/// of strings.
///
/// If there is no common prefix, return an empty string "".
///
///  
///
/// Example 1:
///
/// Input: strs = ["flower","flow","flight"]
/// Output: "fl"
///
/// Example 2:
///
/// Input: strs = ["dog","racecar","car"]
/// Output: ""
/// Explanation: There is no common prefix among the input strings.

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut i = 0;
        loop {
            let mut current_byte = None;
            for str in &strs {
                let byte = str.as_bytes().get(i).copied();
                if let Some(byte) = byte {
                    if let Some(current_byte) = current_byte {
                        if byte != current_byte {
                            return strs[0][..i].to_string();
                        }
                    } else {
                        current_byte = Some(byte);
                    }
                } else {
                    return strs[0][..i].to_string();
                }
            }
            i += 1;
        }
    }
}

pub fn main() {
    println!("{}", Solution::longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]));
}
