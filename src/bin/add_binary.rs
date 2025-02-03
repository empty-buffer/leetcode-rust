/*
Given two binary strings a and b, return their sum as a binary string.

Example 1:

Input: a = "11", b = "1"
Output: "100"
Example 2:

Input: a = "1010", b = "1011"
Output: "10101"
*/

/*
1. Function Declaration:
```rust
fn add_binary(a: String, b: String) -> String
```
- Takes two binary numbers as strings
- Returns their sum as a string

2. Initial Setup:
```rust
let mut result = String::new();
let mut carry = 0;
```
- Creates an empty string to store the result
- Initializes a carry variable to track overflow

3. Input Processing:
```rust
let a_chars: Vec<char> = a.chars().rev().collect();
let b_chars: Vec<char> = b.chars().rev().collect();
```
- Converts input strings to vectors of characters
- Reverses them for easier processing (starting from least significant bit)

4. Main Processing Loop:
```rust
let max_len = a_len.max(b_len);
for i in 0..max_len {
```
- Iterates through all digits, using the length of the longer number

5. Digit Extraction:
```rust
let digit_a = if i < a_len {
    a_chars[i].to_digit(10).unwrap()
} else {
    0
};
```
- Gets digit at position i if available, otherwise uses 0
- Converts char to numeric digit (0 or 1)
- Same process for digit_b

6. Addition Logic:
```rust
let sum = digit_a + digit_b + carry;
carry = sum / 2;
result.push(char::from_digit(sum % 2, 10).unwrap());
```
- Adds current digits and carry
- New carry is sum divided by 2 (1 if sum ≥ 2)
- Pushes remainder (sum % 2) to result

7. Final Carry Check:
```rust
if carry > 0 {
    result.push('1');
}
```
- Adds final carry bit if necessary

8. Result Formation:
```rust
result.chars().rev().collect()
```
- Reverses the result string (as we built it backward)
- Returns final string

Example:
```
Adding "1101" + "1011"
Step by step:
1+1=2 -> carry=1, digit=0
0+1+1=2 -> carry=1, digit=0
1+0+1=2 -> carry=1, digit=0
1+1+1=3 -> carry=1, digit=1
Final carry=1
Result: "11000"
```

This implementation handles binary addition properly by:
- Processing digits right-to-left
- Managing carries correctly
- Handling numbers of different lengths
- Producing the correct binary sum as a string

*/

fn main() {
    println!("{}", Solution::add_v2("110".to_owned(), "11".to_owned()))
}

struct Solution {}

impl Solution {
    fn add_binary(a: String, b: String) -> String {
        let mut result = String::new();
        let mut carry = 0;

        // Convert strings to char vectors and reverse them for easier processing
        let a_chars: Vec<char> = a.chars().rev().collect();
        let b_chars: Vec<char> = b.chars().rev().collect();

        // Get lengths
        let a_len = a_chars.len();
        let b_len = b_chars.len();

        // Process all digits
        let max_len = a_len.max(b_len);

        for i in 0..max_len {
            // Get digits or 0 if we've run out of digits
            let digit_a = if i < a_len {
                a_chars[i].to_digit(10).unwrap()
            } else {
                0
            };
            let digit_b = if i < b_len {
                b_chars[i].to_digit(10).unwrap()
            } else {
                0
            };

            // Add digits and carry
            let sum = digit_a + digit_b + carry;

            // Calculate new carry and digit to append
            carry = sum / 2;
            result.push(char::from_digit(sum % 2, 10).unwrap());
        }

        // Don't forget the final carry if there is one
        if carry > 0 {
            result.push('1');
        }

        // Reverse the result and return
        result.chars().rev().collect()
    }

    fn add_v2(a: String, b: String) -> String {
        format!(
            "{:b}",
            u128::from_str_radix(&a, 2).unwrap() + u128::from_str_radix(&b, 2).unwrap()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[derive(Clone, Debug)]
    struct TestCase {
        name: &'static str,
        str_a: String,
        str_b: String,
        expected: String,
    }

    impl TestCase {
        fn run(&self) {
            let result = Solution::add_binary(self.str_a.clone(), self.str_b.clone());
            assert_eq!(
                result, self.expected,
                "Test case '{}' failed: expected {}, got {}",
                self.name, self.expected, result
            );
        }
    }
    #[test]
    fn add_binary() {
        let test_cases = vec![TestCase {
            name: "true",
            str_a: String::from("11"),
            str_b: String::from("11"),
            expected: String::from("1101"),
        }];

        for tc in &test_cases {
            tc.run();
        }
    }
}
