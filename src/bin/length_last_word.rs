/*
Given a string s consisting of words and spaces, return the length of the last word in the string.

A word is a maximal substring consisting of non-space characters only.


Example 1:

Input: s = "Hello World"
Output: 5
Explanation: The last word is "World" with length 5.
Example 2:

Input: s = "   fly me   to   the moon  "
Output: 4
Explanation: The last word is "moon" with length 4.
Example 3:

Input: s = "luffy is still joyboy"
Output: 6
Explanation: The last word is "joyboy" with length 6.

*/
fn main() {
    let text = String::from("Hellow  world    turn over  ");

    println!("{}", Solution::run(text))
}

struct Solution {}

impl Solution {
    fn run(text: String) -> i32 {
        let words: Vec<String> = text.split_whitespace().map(String::from).collect();

        words[words.len() - 1].len() as i32
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
