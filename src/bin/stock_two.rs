/*
You are given an integer array prices where prices[i] is the price of a given stock on the ith day.

On each day, you may decide to buy and/or sell the stock. You can only hold at most one share of the stock at any time. However, you can buy it then immediately sell it on the same day.

Find and return the maximum profit you can achieve.

Example 1:

Input: prices = vec![6, 1, 3, 2, 4, 7]
Output: 7
Explanation: Buy on day 2 (price = 1) and sell on day 2 (price = 3), profit = 3-1 = 2.
Then buy on day 4 (price = 2) and sell on day 6 (price = 7), profit = 7-3 = 4.
Total profit is 7.

Example 3:

Input: prices = [7,6,4,3,1]
Output: 0
Explanation: There is no way to make a positive profit, so we never buy the stock to achieve the maximum profit of 0.

*/

use std::os::unix::process;

struct Solution {}

impl Solution {
    fn max_profit(prices: Vec<i32>) -> i32 {
        println!("{:?}", prices);
        let mut profit = 0;

        for i in 1..prices.len() {
            if prices[i] > prices[i - 1] {
                profit += prices[i] - prices[i - 1];
            }
        }

        profit
    }

    fn max_profit_park(prices: Vec<i32>) -> i32 {
        println!("{:?}", prices);
        if prices.is_empty() {
            return 0;
        }

        let mut profit = 0;
        let mut valley = prices[0];
        let mut peak = prices[0];
        let mut i = 0;

        while i < prices.len() - 1 {
            // Find valley (local minimum)
            while i < prices.len() - 1 && prices[i] >= prices[i + 1] {
                println!("{}", prices[i]);
                i += 1;
            }
            valley = prices[i];

            // Find peak (local maximum)
            while i < prices.len() - 1 && prices[i] <= prices[i + 1] {
                i += 1;
            }
            peak = prices[i];

            // Add profit from this valley-peak pair
            profit += peak - valley;
        }

        profit
    }
}

fn main() {
    let test = vec![6, 1, 3, 2, 4, 7];
    Solution::max_profit_park(test);
}

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
            let result = Solution::max_profit_park(self.prices.clone());
            assert_eq!(
                result, self.expected,
                "Test case '{}' failed: expected {}, got {}",
                self.name, self.expected, result
            );
        }
    }

    #[test]
    fn stock_two() {
        let test_cases = vec![
            TestCase {
                name: "should return profit when possible",
                prices: vec![7, 1, 5, 3, 6, 4],
                expected: 7,
            },
            TestCase {
                name: "should return 0 when no profit possible",
                prices: vec![7, 6, 4, 3, 1],
                expected: 0,
            },
            TestCase {
                name: "should handle simple case",
                prices: vec![1, 2, 3, 4, 5],
                expected: 4,
            },
            TestCase {
                name: "should handle simple case",
                prices: vec![6, 1, 3, 2, 4, 7],
                expected: 7,
            },
        ];

        for tc in test_cases {
            tc.run();
        }
    }
}
