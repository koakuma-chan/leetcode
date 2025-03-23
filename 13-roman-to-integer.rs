/// Roman numerals are represented by seven different symbols: I, V, X, L, C, D
/// and M.
///
/// Symbol       Value
/// I             1
/// V             5
/// X             10
/// L             50
/// C             100
/// D             500
/// M             1000
///
/// For example, 2 is written as II in Roman numeral, just two ones added
/// together. 12 is written as XII, which is simply X + II. The number 27 is
/// written as XXVII, which is XX + V + II.
///
/// Roman numerals are usually written largest to smallest from left to right.
/// However, the numeral for four is not IIII. Instead, the number four is
/// written as IV. Because the one is before the five we subtract it making
/// four. The same principle applies to the number nine, which is written as IX.
/// There are six instances where subtraction is used:
///
///     I can be placed before V (5) and X (10) to make 4 and 9.
///     X can be placed before L (50) and C (100) to make 40 and 90.
///     C can be placed before D (500) and M (1000) to make 400 and 900.
///
/// Given a roman numeral, convert it to an integer.

pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;

        let mut iterator = s.chars().peekable();

        while let Some(c) = iterator.next() {
            let value = match c {
                'I' => match iterator.peek().copied() {
                    Some('V') => {
                        iterator.next();
                        4
                    }
                    Some('X') => {
                        iterator.next();
                        9
                    }
                    _ => 1,
                },
                'V' => 5,
                'X' => match iterator.peek().copied() {
                    Some('L') => {
                        iterator.next();
                        40
                    }
                    Some('C') => {
                        iterator.next();
                        90
                    }
                    _ => 10,
                },
                'L' => 50,
                'C' => match iterator.peek().copied() {
                    Some('D') => {
                        iterator.next();
                        400
                    }
                    Some('M') => {
                        iterator.next();
                        900
                    }
                    _ => 100,
                },
                'D' => 500,
                'M' => 1000,
                _ => unreachable!(),
            };

            result += value;
        }

        result
    }
}

pub fn main() {
    println!("{}", Solution::roman_to_int("MCMXCIV".to_string()));
}
