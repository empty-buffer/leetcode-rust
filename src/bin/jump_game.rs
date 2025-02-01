use std::usize;

/*
You are given an integer array nums. You are initially positioned at the array's first index,
and each element in the array represents your maximum jump length at that position.

Example 1:

Input: nums = [2,3,1,1,4]
Output: true
Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.
Example 2:

Input: nums = [3,2,1,0,4]
Output: false
Explanation: You will always arrive at index 3 no matter what. Its maximum jump length is 0, which makes it impossible to reach the last index.

*/
fn main() {
    let nums = vec![3, 2, 1, 0, 4];
    let nums2 = vec![2, 3, 1, 0, 4];

    println!("{:?}", Solution::run(nums2));

    println!("{:?}", Solution::run_v2(nums));
}

struct Solution {}

impl Solution {
    fn run(nums: Vec<i32>) -> bool {
        if nums.is_empty() || nums.len() < 2 {
            return true;
        }

        let mut max_reach = 0;

        // We only need to check up to the second-to-last element
        // because if we can reach the last element, we're done
        for i in 0..nums.len() - 1 {
            // If we can't reach the current position, return false
            if i > max_reach {
                return false;
            }

            // Update the maximum reachable position
            max_reach = max_reach.max(i + nums[i] as usize);

            // If we can already reach the last index, return true
            if max_reach >= nums.len() - 1 {
                return true;
            }
        }

        // Check if we can reach the last position
        max_reach >= nums.len() - 1
    }

    fn run_v2(nums: Vec<i32>) -> bool {
        if nums.is_empty() || nums.len() < 2 {
            return true;
        }

        for i in 0..nums.len() - 1 {
            if i == i + nums[i] as usize {
                return false;
            }

            if Self::jump_recursive(&nums, i) {
                return true;
            }
        }

        false
    }

    fn run_v3(nums: Vec<i32>) -> bool {
        let mut jump = 0;

        for &n in nums.iter().take(nums.len() - 1) {
            jump = jump.max(n) - 1;
            if jump < 0 {
                return false;
            }
        }

        true
    }

    fn jump_recursive(nums: &Vec<i32>, position: usize) -> bool {
        if position == position + nums[position] as usize {
            return false;
        }

        if position + nums[position] as usize >= nums.len() - 1 {
            return true;
        }

        Self::jump_recursive(nums, nums[position] as usize + position)
    }

    fn greedy(nums: Vec<i32>) -> bool {
        let mut m = 0;
        for (i, val) in nums.iter().enumerate() {
            if i > m {
                return false;
            }
            let val = *val as usize;
            m = std::cmp::max(m, i + val);
        }
        return true;
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
        expected: bool,
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

        fn run_v2(&self) {
            let result = Solution::run_v2(self.prices.clone());
            assert_eq!(
                result, self.expected,
                "Test case '{}' failed: expected {}, got {}",
                self.name, self.expected, result
            );
        }
    }
    #[test]
    fn jump_game() {
        let test_cases = vec![
            TestCase {
                name: "true",
                prices: vec![2, 3, 1, 1, 4],
                expected: true,
            },
            TestCase {
                name: "true",
                prices: vec![3, 2, 1, 0, 4],
                expected: false,
            },
            TestCase {
                name: "0,2,3",
                prices: vec![0, 2, 3],
                expected: false,
            },
        ];

        for tc in &test_cases {
            tc.run();
        }

        for tc in test_cases {
            tc.run_v2();
        }
    }
}
