/// Given a string s consisting of words and spaces, return the length of the
/// last word in the string.
///
/// A word is a maximal substring consisting of non-space characters only.

struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        use std::convert::TryInto;
        s.split(' ')
            .filter(|word| word.len() > 0)
            .last()
            .map(|word| word.len())
            .unwrap()
            .try_into()
            .unwrap()
    }
}

pub fn main() {
    println!(
        "{}",
        Solution::length_of_last_word("   fly me   to   the moon  ".to_string())
    );
}
