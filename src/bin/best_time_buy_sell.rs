/*
You are given an array prices where prices[i] is the price of a given stock on the ith day.

You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.

Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.
*/

use std::usize;

struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min: usize = 0;
        let mut max: usize = 1;

        for i in 1..prices.len() {
            if prices[i] < prices[min] {
                min = i;
            }

            if prices[i] > prices[max] {
                max = i;
            }
        }

        if min < max {
            return prices[max] - prices[min];
        }

        0
    }

    pub fn max_profit_v1(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .fold((i32::MAX, 0), |(min_price, max_profit), &price| {
                (min_price.min(price), max_profit.max(price - min_price))
            })
            .1
    }

    pub fn max_profit_v2(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::MAX;
        let mut max_profit = 0;

        for &price in prices.iter() {
            min_price = min_price.min(price);
            max_profit = max_profit.max(price - min_price);
        }

        max_profit
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::Solution;

    pub type Result<T> = core::result::Result<T, Error>;
    pub type Error = Box<dyn std::error::Error>;

    #[derive(Clone, Debug)]
    struct TestCase {
        name: &'static str,
        prices: Vec<i32>,
        expected: i32,
    }

    impl TestCase {
        fn run(&self) {
            let result = Solution::max_profit_v2(self.prices.clone());
            assert_eq!(
                result, self.expected,
                "Test case '{}' failed: expected {}, got {}",
                self.name, self.expected, result
            );
        }
    }

    #[test]
    fn test_buy_sell() {
        let test_cases = vec![
            TestCase {
                name: "should return profit when possible",
                prices: vec![7, 1, 5, 3, 6, 4],
                expected: 5,
            },
            TestCase {
                name: "should return 0 when no profit possible",
                prices: vec![7, 6, 4, 3, 1],
                expected: 0,
            },
            TestCase {
                name: "should handle simple case",
                prices: vec![1, 2],
                expected: 1,
            },
        ];

        for tc in test_cases {
            tc.run();
        }
    }
}
