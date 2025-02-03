fn main() {
    todo!()
}

struct Solution {}

impl Solution {
    fn run(text: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[derive(Clone, Debug)]
    struct TestCase {
        name: &'static str,
        text: String,
        expected: i32,
    }

    impl TestCase {
        fn run(&self) {
            let result = Solution::run(self.text.clone());
            assert_eq!(
                result, self.expected,
                "Test case '{}' failed: expected {}, got {}",
                self.name, self.expected, result
            );
        }
    }
    #[test]
    fn lenght_last_word() {
        let test_cases = vec![TestCase {
            name: "true",
            text: String::from("  Dog, goes wild    and"),
            expected: 3,
        }];

        for tc in &test_cases {
            tc.run();
        }
    }
}
