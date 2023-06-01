pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut value = x;
        let mut inverse = 0;

        while value > 0 {
            let digit = value % 10;
            inverse *= 10;
            inverse += digit;
            value /= 10;
        }
        x == inverse
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome() {
        assert!(Solution::is_palindrome(12321));
        assert!(Solution::is_palindrome(1));
        assert!(Solution::is_palindrome(11));
        assert!(Solution::is_palindrome(12344321));
    }

    #[test]
    fn test_not_palindrome() {
        assert!(!Solution::is_palindrome(12));
        assert!(!Solution::is_palindrome(-121));
        assert!(!Solution::is_palindrome(10));
    }
}
