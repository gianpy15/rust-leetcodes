use regex::Regex;

pub struct Solution;

impl Solution {
    pub fn is_palindrome_regex(s: String) -> bool {
        let regex = Regex::new(r"\w+").unwrap();
        let cleaned_str = regex
            .find_iter(&s)
            .into_iter()
            .map(|s| s.as_str())
            .collect::<String>()
            .to_lowercase();
        let reversed = cleaned_str.chars().rev().collect::<String>();
        reversed == cleaned_str
    }

    pub fn is_palindrome(s: String) -> bool {
        let clean_str = Self::clean_string(s);
        let reversed = clean_str.chars().rev().collect::<String>();
        clean_str == reversed
    }

    pub fn clean_string(s: String) -> String {
        s.to_lowercase().chars().filter(|char| (&'a' <= char && char <= &'z') || (&'0' <= char && char <= &'9') ).collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome() {
        assert!(Solution::is_palindrome(
            "A man, a plan, a canal: Panama".to_string()
        ));
        assert!(Solution::is_palindrome_regex(
            "A man, a plan, a canal: Panama".to_string()
        ));
    }

    #[test]
    fn test_non_palindrome_2() {
        assert!(!Solution::is_palindrome("race a car".to_string()));
        assert!(!Solution::is_palindrome_regex("race a car".to_string()));
    }

    #[test]
    fn test_palindrome_3() {
        assert!(Solution::is_palindrome(" ".to_string()));
        assert!(Solution::is_palindrome_regex(" ".to_string()));
    }

    #[test]
    fn test_non_palindrome_3() {
        assert!(!Solution::is_palindrome("0P".to_string()));
        assert!(!Solution::is_palindrome_regex("0P".to_string()));
    }

    #[test]
    fn test_not_palindrome() {
        assert!(!Solution::is_palindrome("abcd".to_string()));
        assert!(!Solution::is_palindrome_regex("abcd".to_string()));
    }
}
