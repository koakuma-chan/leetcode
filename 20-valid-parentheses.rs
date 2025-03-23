/// Given a string s containing just the characters '(', ')', '{', '}', '[' and
/// ']', determine if the input string is valid.
///
/// An input string is valid if:
///
///     Open brackets must be closed by the same type of brackets.
///     Open brackets must be closed in the correct order.
///     Every close bracket has a corresponding open bracket of the same type.

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::with_capacity(s.len());
        for c in s.chars() {
            match c {
                ')' => {
                    if stack.pop() != Some('(') {
                        return false;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        return false;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        return false;
                    }
                }
                _ => stack.push(c),
            }
        }
        stack.is_empty()
    }
}

pub fn main() {
    println!("{}", Solution::is_valid("([])".to_string()));
}
