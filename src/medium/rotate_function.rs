pub struct Solution;

impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let multipliers: Vec<i32> = (0..nums.len() as i32).collect();
        let tot_sum: i32 = nums.iter().sum();
        let mut product = nums.iter().zip(multipliers.iter()).map(|(v1, v2)| v1 * v2).sum();
        let mut max_val = product;

        for idx in (1..nums.len()).rev() {
            let new_value = product + tot_sum - nums[idx] * nums.len() as i32;
            if new_value > max_val {
                max_val = new_value;
            }
            product = new_value;
        }
        max_val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        assert!(Solution::max_rotate_function(vec![4,3,2,6]) == 26);
    }

    #[test]
    fn test_rotate_2() {
        assert!(Solution::max_rotate_function(vec![100]) == 0);
    }
}
