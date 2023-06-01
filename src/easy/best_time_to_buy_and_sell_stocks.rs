use std::cmp;

pub struct Solution;

impl Solution {
    pub fn recursive_max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 1 {
            return 0;
        }
        let max_value = prices[1..].iter().max().unwrap();
        let val = prices.get(0).unwrap();
        cmp::max(max_value - val, Self::max_profit(prices[1..].to_vec()))
    }

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_prices = vec![prices.last().unwrap()];
        for price in prices[..prices.len() - 1].iter().rev() {
            max_prices.push(cmp::max(price, max_prices.last().unwrap()));
        }
        max_prices.reverse();

        let mut max_revenue = 0;
        for (index, price) in prices.iter().enumerate() {
            max_revenue = cmp::max(max_revenue, *max_prices.get(index).unwrap() - price)
        }

        max_revenue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profits() {
        assert!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]) == 5);
    }

    #[test]
    fn test_profits_2() {
        assert!(Solution::max_profit(vec![7, 6, 4, 3, 1]) == 0);
    }
}
