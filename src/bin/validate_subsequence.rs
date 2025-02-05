fn main() {}

struct Solution {}

impl Solution {
    fn sub_sequence_v1(sq: Vec<i32>, sub: Vec<i32>) -> bool {
        let mut current_index = 0;

        for n in 0..sub.len() {
            if current_index >= sq.len() - 1 {
                return false;
            }

            for j in current_index..sq.len() {
                if sq[j] == sub[n] {
                    if n == sub.len() - 1 {
                        return true;
                    }

                    current_index = j + 1;
                    break;
                }
                current_index = j
            }
        }

        false
    }

    // O(n) time| O(1) space
    fn sub_sequence_v2(seq: Vec<i32>, sub_seq: Vec<i32>) -> bool {
        let mut seq_idx = 0;

        for &val in &seq {
            if seq_idx == seq.len() {
                return true;
            }

            if val == sub_seq[seq_idx] {
                seq_idx += 1;
            }

            // while seq_idx < seq.len() && seq[seq_idx] != sub_val {
            //     seq_idx += 1;
            // }

            // if seq_idx >= seq.len() {
            //     return false;
            // }
        }

        seq_idx == sub_seq.len()
    }

    // O(n) time| O(1) space
    fn sub_sequence_two_pointer(seq: Vec<i32>, sub_seq: Vec<i32>) -> bool {
        let (mut s, mut t) = (0, 0);

        while s < seq.len() && t < sub_seq.len() {
            if seq[s] == sub_seq[t] {
                t += 1;
            }
            s += 1;
        }
        t == sub_seq.len()
    }
}

#[cfg(test)]

mod tests {
    use crate::Solution;

    #[derive(Clone, Debug)]
    struct TestCase {
        name: &'static str,
        sequence: Vec<i32>,
        sub_seq: Vec<i32>,
        expected: bool,
    }

    impl TestCase {
        fn run(&self) {
            {
                let result = Solution::sub_sequence_v2(self.sequence.clone(), self.sub_seq.clone());
                println!("sub_sequence_v2: {}", self.name);
                assert_eq!(
                    result, self.expected,
                    "Test case '{}' failed: expected {}, got {}",
                    self.name, self.expected, result
                );
            }
            {
                let result =
                    Solution::sub_sequence_two_pointer(self.sequence.clone(), self.sub_seq.clone());
                println!("sub_sequence_two_pointer: {}", self.name);
                assert_eq!(
                    result, self.expected,
                    "Test case '{}' failed: expected {}, got {}",
                    self.name, self.expected, result
                );
            }
        }
    }
    #[test]
    fn sub_sequence() {
        let test_cases = vec![
            TestCase {
                name: "true",
                sequence: vec![5, 1, 22, 25, 6, -1, 8, 10],
                sub_seq: vec![1, 6, -1, 10],
                expected: true,
            },
            TestCase {
                name: "false",
                sequence: vec![5, 1, 22, 25, 6, -1, 8, 10],
                sub_seq: vec![1, 3, -1, 10],
                expected: false,
            },
        ];

        for tc in &test_cases {
            tc.run();
        }
    }
}
