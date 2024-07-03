/*
You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers
m and n, representing the number of elements in nums1 and nums2 respectively.
Merge nums1 and nums2 into a single array sorted in non-decreasing order.
The final sorted array should not be returned by the function, but instead be stored inside the array nums1.
To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged,
and the last n elements are set to 0 and should be ignored. nums2 has a length of n.
Example 1:

Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
Output: [1,2,2,3,5,6]
Explanation: The arrays we are merging are [1,2,3] and [2,5,6].
The result of the merge is [1,2,2,3,5,6] with the underlined elements coming from nums1.
Example 2:

Input: nums1 = [1], m = 1, nums2 = [], n = 0
Output: [1]
Explanation: The arrays we are merging are [1] and [].
The result of the merge is [1].
Example 3:

Input: nums1 = [0], m = 0, nums2 = [1], n = 1
Output: [1]
Explanation: The arrays we are merging are [] and [1].
The result of the merge is [1].
Note that because m = 0, there are no elements in nums1. The 0 is only there to ensure the merge result can fit in nums1.
 */

fn main() {
    let (mut arr1, mut arr2) = (vec![1,2,3,0,0,0], vec![2,5,6]);
    Solution::merge_sorted_array(&mut arr1, 3, &mut arr2, 3);
}

struct Solution;
impl Solution {
    pub fn merge_sorted_array(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32){
        nums1.truncate(m as usize);
        nums1.extend_from_slice(&nums2[..n as usize]);
        nums1.sort();
    }

    pub fn merge_sorted_array_without_fn(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32){
        let (mut m, mut n) = (m as usize, n as usize);
        while n > 0 {
            if m > 0 && nums1[m - 1] > nums2[n - 1] {
                nums1[m + n - 1] = nums1[m - 1];
                m -= 1;
            } else {
                nums1[m + n - 1] = nums2[n - 1];
                n -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    pub type Result<T> = core::result::Result<T, Error>;
    pub type Error = Box<dyn std::error::Error>;

struct TestCase{
    arr1: Vec<i32>,
    arr2: Vec<i32>,
    d1: i32,
    d2: i32,
    result: Vec<i32>
}
    #[test]
    fn test_find_majority() -> Result<()> {
        let test_cases:Vec<TestCase> = vec![
            TestCase {
                arr1: vec![1, 2, 3, 0, 0, 0],
                arr2: vec![2, 5, 6],
                d1: 3,
                d2: 3,
                result: vec![1, 2, 2, 3, 5, 6]
            },
            TestCase {
                arr1: vec![1],
                arr2: vec![0],
                d1: 1,
                d2: 0,
                result: vec![1]
            },
        ];



        for mut test in test_cases{
            Solution::merge_sorted_array(&mut test.arr1, test.d1, &mut test.arr2, test.d2);
            assert_eq!(test.arr1, test.result, "Test failed output: {:?}", test.arr1);
        }

        Ok(())
    }

    #[test]
    fn merge_sorted_array_without_fn() -> Result<()> {
        let test_cases:Vec<TestCase> = vec![
            TestCase {
                arr1: vec![1, 2, 3, 0, 0, 0],
                arr2: vec![2, 5, 6],
                d1: 3,
                d2: 3,
                result: vec![1, 2, 2, 3, 5, 6]
            },
            TestCase {
                arr1: vec![1],
                arr2: vec![0],
                d1: 1,
                d2: 0,
                result: vec![1]
            },
        ];



        for mut test in test_cases{
            Solution::merge_sorted_array_without_fn(&mut test.arr1, test.d1, &mut test.arr2, test.d2);
            assert_eq!(test.arr1, test.result, "Test failed output: {:?}", test.arr1);
        }

        Ok(())
    }
}