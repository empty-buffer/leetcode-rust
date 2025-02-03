fn main() {
    let d = vec![9, 9, 9];

    println!("Result v1: {:?}", Solution::run(d.clone()));
    println!("Result v2: {:?}", Solution::version_2(d.clone()));
    println!("Result map: {:?}", Solution::version_map(d));
}

struct Solution {}

impl Solution {
    fn run(mut digits: Vec<i32>) -> Vec<i32> {
        let mut all_nines = true;

        let mut i = digits.len();

        while i > 0 {
            i -= 1;

            if digits[i] == 9 {
                digits[i] = 0;
                continue;
            } else {
                digits[i] += 1;
                all_nines = false;
                break;
            }
        }

        if all_nines {
            digits.insert(0, 1);
        }

        digits
    }

    fn version_map(mut digits: Vec<i32>) -> Vec<i32> {
        let mut insert_digit = true;

        let _: Vec<i32> = digits
            .iter_mut()
            .rev()
            .map(|x| {
                match *x == 9 {
                    true => *x = 0,
                    false => {
                        *x += 1;
                        insert_digit = false
                    }
                }
                *x
            })
            .collect();

        if insert_digit {
            digits.insert(0, 1);
        }

        digits
    }

    fn version_2(mut digits: Vec<i32>) -> Vec<i32> {
        for x in digits.iter_mut().rev() {
            match *x == 9 {
                true => *x = 0,
                false => {
                    *x += 1;
                    return digits;
                }
            }
        }
        digits.insert(0, 1);

        digits
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[derive(Clone, Debug)]
    struct TestCase {
        name: &'static str,
        digits: Vec<i32>,
        expected: Vec<i32>,
    }

    impl TestCase {
        fn run(&self) {
            let result = Solution::run(self.digits.clone());
            assert_eq!(
                result, self.expected,
                "Test case '{}' failed: expected {:?}, got {:?}",
                self.name, self.expected, result
            );
        }
    }
    #[test]
    fn lenght_last_word() {
        let test_cases = vec![TestCase {
            name: "9,9,9",
            digits: vec![9, 9, 9],
            expected: vec![1, 0, 0, 0],
        }];

        for tc in &test_cases {
            tc.run();
        }
    }
}
