/*Example 1:

Given an integer array nums and an integer val, remove all occurrences of val in nums in-place. The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.

Consider the number of elements in nums which are not equal to val be k, to get accepted, you need to do the following things:

Change the array nums such that the first k elements of nums contain the elements which are not equal to val. The remaining elements of nums are not important as well as the size of nums.
Return k.

Input: nums = [3,2,2,3], val = 3
Output: 2, nums = [2,2,_,_]
Explanation: Your function should return k = 2, with the first two elements of nums being 2.
It does not matter what you leave beyond the returned k (hence they are underscores).
Example 2:

Input: nums = [0,1,2,2,3,0,4,2], val = 2
Output: 5, nums = [0,1,4,0,3,_,_,_]
Explanation: Your function should return k = 5, with the first five elements of nums containing 0, 0, 1, 3, and 4.
Note that the five elements can be returned in any order.
It does not matter what you leave beyond the returned k (hence they are underscores).
 */

fn main() {
    let mut nums: Vec<i32> = vec![3, 2, 2, 3];

    let val = 3;
    Solution::remove_element(&mut nums, val);

    println!("{}, array {:?}", nums.len(), nums)
}

struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&x| x != val);
        nums.clone().len() as i32
    }

    pub fn remove_element_without_methods(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut count = 0;
        let num = nums.clone();
        for i in num.iter() {
            if *i != val {
                nums[count as usize] = *i;
                count += 1;
            }
        }
        println!("{}", count);
        count
    }

    pub fn remove_element_without_methods_v2(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut matched = 0;
        for cur in 0..nums.len() {
            match nums[cur] == val {
                true => (),
                false => {
                    nums[matched] = nums[cur];
                    matched += 1;
                }
            };
        }
        println!("{}", matched);
        return matched as i32;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    pub type Result<T> = core::result::Result<T, Error>;
    pub type Error = Box<dyn std::error::Error>;

    #[derive(Clone, Debug)]
    struct TestCase {
        nums: Vec<i32>,
        val: i32,
        result: i32
    }
    #[test]
    fn remove_element() -> Result<()> {
        let test_cases:Vec<TestCase> = vec![
            TestCase {
                nums: vec![1, 2, 3, 0, 0, 0],
                val: 0,
                result: 3
            },
        ];


        for mut test in test_cases.clone(){
            let res = Solution::remove_element(&mut test.nums, test.val);
            assert_eq!(res, test.result, "Test failed output: {:?}", test.nums);
        }

        for mut test in test_cases.clone(){
            let res = Solution::remove_element_without_methods_v2(&mut test.nums, test.val);
            assert_eq!(res, test.result, "Test failed output: {:?}", test.nums);
        }
        for mut test in test_cases.clone(){
            let res = Solution::remove_element_without_methods(&mut test.nums, test.val);
            assert_eq!(res, test.result, "Test failed output: {:?}", test.nums);
        }

        Ok(())
    }
}