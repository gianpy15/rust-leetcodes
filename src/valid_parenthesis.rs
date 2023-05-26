pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for parenthesis in s.chars() {
            let check = match parenthesis {
                '(' => {
                    stack.push(')');
                    true
                }
                '[' => {
                    stack.push(']');
                    true
                }
                '{' => {
                    stack.push('}');
                    true
                }

                ')' => Solution::pop_and_match(&mut stack, parenthesis),
                ']' => Solution::pop_and_match(&mut stack, parenthesis),
                '}' => Solution::pop_and_match(&mut stack, parenthesis),

                _ => return false,
            };
            if !check {
                return check;
            }
        }
        stack.is_empty()
    }

    fn pop_and_match(stack: &mut Vec<char>, parenthesis: char) -> bool {
        match stack.pop() {
            Some(char) => char == parenthesis,
            None => false,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_parenthesis() {
        assert!(Solution::is_valid("[]".to_string()));
        assert!(Solution::is_valid("[[()]]{}{{([])()}{}}".to_string()));
    
    }

    #[test]
    fn test_not_valid_parenthesis() {
        assert!(!Solution::is_valid("[".to_string()));
        assert!(!Solution::is_valid("[)".to_string()));
        assert!(!Solution::is_valid("ciao".to_string()));
        assert!(!Solution::is_valid("[[()]]{}{{([])()}{}}}".to_string()));
        assert!(!Solution::is_valid("[[()]]{}{([])()}{}}".to_string()));
    }
}