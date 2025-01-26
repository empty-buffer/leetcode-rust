/*
Given an integer array nums sorted in non-decreasing order, remove some duplicates in-place such that each unique element appears at most twice.
The relative order of the elements should be kept the same.

Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the first part of the array nums.
More formally, if there are k elements after removing the duplicates, then the first k elements of nums should hold the final result.
It does not matter what you leave beyond the first k elements.
Return k after placing the final result in the first k slots of nums.

Do not allocate extra space for another array. You must do this by modifying the input array in-place with O(1) extra memory.

*/

fn main() {}

struct Solution;

//vec![1, 1, 1, 1, 1, 1, 2, 2, 3];
//vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
// 2  3
// i
// p
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut repeat: u8 = 0;
        let mut index: usize = 1;

        for p in 1..nums.len() {
            if nums[p] == nums[p - 1] && repeat < 1 {
                nums[index] = nums[p];
                index += 1;
                repeat += 1;
            }
            if nums[p] != nums[p - 1] {
                nums[index] = nums[p];
                index += 1;
                repeat = 0;
            }
        }

        println!("{:?}", nums);
        index as i32
    }
    pub fn remove_duplicates_flat_map(nums: &mut Vec<i32>) -> i32 {
        *nums = nums
            .iter()
            .enumerate()
            .flat_map(|(i, &n)| {
                if i > 1 && nums[i - 2] == n {
                    None
                } else {
                    Some(n)
                }
            })
            .collect();
        println!("{:?}", nums);

        nums.len() as i32
    }

    pub fn remove_duplicates_filter_map(nums: &mut Vec<i32>) -> i32 {
        *nums = nums
            .iter()
            .enumerate()
            .filter_map(|(i, &n)| {
                if i > 1 && nums[i - 2] == n {
                    None
                } else {
                    Some(n)
                }
            })
            .collect();
        println!("{:?}", nums);
        nums.len() as i32
    }
    pub fn remove_duplicates_two_pointer(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }

        let mut write_index = 2;

        for read_index in 2..nums.len() {
            if nums[read_index] != nums[write_index - 2] {
                nums[write_index] = nums[read_index];
                write_index += 1;
            }
        }

        println!("{:?}", nums);
        write_index as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    pub type Result<T> = core::result::Result<T, Error>;
    pub type Error = Box<dyn std::error::Error>;

    struct TestCase {
        input: Vec<i32>,
        expected_result: i32,
    }

    #[test]
    fn remove_dupl_from_sorted_2() -> Result<()> {
        let case_1 = TestCase {
            input: vec![0, 0, 1, 1, 1, 1, 2, 3, 3],
            expected_result: 7,
        };
        let case_2 = TestCase {
            input: vec![1, 1, 1, 1, 1, 1, 2, 2, 3],
            expected_result: 5,
        };

        let mut test_cases = vec![case_1, case_2];

        for case in test_cases.iter_mut() {
            let r = Solution::remove_duplicates_flat_map(&mut case.input);
            assert_eq!(case.expected_result, r);
        }

        Ok(())
    }
}
