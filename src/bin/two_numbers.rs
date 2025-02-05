use std::collections::HashMap;

fn main() {
    let nums: Vec<i32> = vec![3, 5, -4, 8, 11, 1, -1, 6];

    println!("{}", Solution::sorted(nums, 20))
}

struct Solution {}

impl Solution {
    fn sorted(mut nums: Vec<i32>, target: i32) -> bool {
        nums.sort();

        let r = nums.len() - 1;

        Self::sorted_helper(nums, 0, r, target)
    }

    fn sorted_helper(nums: Vec<i32>, l: usize, r: usize, target: i32) -> bool {
        if l > r {
            return false;
        }

        if nums[l] + nums[r] == target {
            return true;
        }

        if nums[l] + nums[r] > target {
            return Self::sorted_helper(nums, l, r - 1, target);
        } else {
            return Self::sorted_helper(nums, l + 1, r, target);
        }
    }

    fn hash_map(nums: Vec<i32>, target: i32) -> bool {
        let mut map: HashMap<i32, bool> = HashMap::new();

        for n in nums {
            let s = target - n;

            if map.contains_key(&s) {
                return true;
            } else {
                map.insert(n, true);
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    // TODO Test

    #[derive(Clone, Debug)]
    struct TestCase {
        name: &'static str,
        nums: Vec<i32>,
        target: i32,
        expected: bool,
    }

    impl TestCase {
        fn run(&self) {
            let result = Solution::hash_map(self.nums.clone(), self.target);
            assert_eq!(
                result, self.expected,
                "Test case '{}' failed: expected {}, got {}",
                self.name, self.expected, result
            );
        }
    }

    #[test]
    fn two_numbers_haspmap() {
        let test_cases = vec![TestCase {
            name: "hash_map",
            nums: vec![3, 5, -4, 8, 11, 1, -1, 6],
            target: 10,
            expected: true,
        }];

        for tc in &test_cases {
            tc.run();
        }
    }
}
