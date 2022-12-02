
use std::cmp::Ordering;

use crate::Solution;

// 9. Palindrome Number
// https://leetcode.com/problems/palindrome-number/

impl Solution {
    fn is_palindrome(mut x: i32) -> bool {
        match x.cmp(&0) {
            Ordering::Equal => true,
            Ordering::Less => false,
            Ordering::Greater => {
                let mut len = 0;
                { // len = x.ilog10()
                    let mut tmp = x;
                    while tmp != 0 {
                        tmp /= 10;
                        len += 1;
                    }
                }
                for i in 0..(len / 2) {
                    if x % 10 != (x / 10i32.pow(len - 2*i - 1)) % 10 { return false }
                    x /= 10;
                }
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_9() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
        assert_eq!(Solution::is_palindrome(1123211), true);
        assert_eq!(Solution::is_palindrome(-10100101), false);
        assert_eq!(Solution::is_palindrome(2147447412), true);
    }
}