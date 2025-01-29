fn main() {
    let prices = vec![7, 6, 4, 3, 1];

    println!("{:?}", prices);
    println!("{}", Solution::run(prices))
}

struct Solution {}

impl Solution {
    fn run(prices: Vec<i32>) -> i32 {
        let by_days = Self::days(&prices);
        let seq = Self::seq(&prices);

        if by_days > seq {
            return by_days;
        }
        seq
    }

    fn seq(prices: &Vec<i32>) -> i32 {
        let mut profit = 0;

        for i in 1..prices.len() {
            if prices[i] > prices[i - 1] {
                profit += prices[i] - prices[i - 1];
            }
        }

        profit
    }

    fn days(prices: &Vec<i32>) -> i32 {
        let mut sold_days: Vec<i32> = Vec::new();

        if prices.is_empty() {
            return 0;
        }

        let mut profit = 0;
        let mut max_price = 0;
        let mut buy_price = prices[0];
        let mut i = 0;

        while i < prices.len() - 1 {
            while i < prices.len() - 1 && prices[i] >= prices[i + 1] {
                i += 1;
            }

            buy_price = prices[i];

            for j in i + 1..prices.len() {
                if prices[j] > buy_price && !sold_days.contains(&prices[j]) {
                    max_price = prices[j];
                }
            }

            if max_price > 0 {
                profit += max_price - buy_price;
                sold_days.push(max_price);
                max_price = 0;
            }
            i += 1;
        }

        profit
    }
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
            let result = Solution::run(self.prices.clone());
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
                name: "Case [7, 1, 5, 3, 6, 4] - should return 7",
                prices: vec![7, 1, 5, 3, 6, 4],
                expected: 7,
            },
            TestCase {
                name: "Case [7, 6, 4, 3, 1] - no profit possible",
                prices: vec![7, 6, 4, 3, 1],
                expected: 0,
            },
            TestCase {
                name: "should handle simple case",
                prices: vec![1, 2, 3, 4, 5],
                expected: 6,
            },
            TestCase {
                name: "should handle simple case",
                prices: vec![6, 1, 3, 2, 4, 7],
                expected: 8,
            },
        ];

        for tc in test_cases {
            tc.run();
        }
    }
}
