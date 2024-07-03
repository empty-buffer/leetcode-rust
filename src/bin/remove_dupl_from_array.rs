/*Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same. Then return the number of unique elements in nums.

Consider the number of unique elements of nums to be k, to get accepted, you need to do the following things:

Change the array nums such that the first k elements of nums contain the unique elements in the order they were present in nums initially. The remaining elements of nums are not important as well as the size of nums.
Return k.

Example 1:

Input: nums = [1,1,2]
Output: 2, nums = [1,2,_]
Explanation: Your function should return k = 2, with the first two elements of nums being 1 and 2 respectively.
It does not matter what you leave beyond the returned k (hence they are underscores).
Example 2:

Input: nums = [0,0,1,1,1,2,2,3,3,4]
Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]
Explanation: Your function should return k = 5, with the first five elements of nums being 0, 1, 2, 3, and 4 respectively.
It does not matter what you leave beyond the returned k (hence they are underscores).*/

fn main() {

}

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.clone_from(&nums.iter().fold(vec![], |mut acc, &num| {
            if acc.last() != Some(&num) {
                acc.push(num);
            }
            acc
        }));
        nums.len() as i32
    }

    pub fn remove_duplicates_v2(nums: &mut Vec<i32>) -> i32 {
        let mut res:Vec<i32> = vec![];
        for v in nums.iter(){
            if res.last() != Some(v){
                res.push(*v)
            }
        }

        println!("{:?}",res);
        res.len() as i32
    }

    pub fn remove_duplicates_v3(nums: &mut [i32]) -> i32 {
        let mut p1 = 1;
        //if you need to start not from the start of array in 1..nums.len()
        for p2 in 1..nums.len() {
            if nums[p2] != nums[p2 - 1] {
                nums[p1] = nums[p2];
                p1 += 1;
            }
        }
        println!("{:?}",nums);
        p1 as i32
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
        result: Vec<i32>,
        result_len: i32
    }
    #[test]
    fn remove_element() -> Result<()> {
        let test_cases:Vec<TestCase> = vec![
            TestCase {
                nums: vec![0,0,1,1,1,2,2,3,3,4],
                result_len: 5,
                result: vec![0,1,2,3,4]
            },
        ];


        for mut test in test_cases.clone(){
            let res = Solution::remove_duplicates(&mut test.nums);
            assert_eq!(res, test.result_len, "Test failed output: {:?}", test.nums);
        }


        for mut test in test_cases.clone(){
            let res = Solution::remove_duplicates_v2(&mut test.nums);
            assert_eq!(res, test.result_len, "Test failed output: {:?}", test.nums);
            
        } 
        
        for mut test in test_cases.clone(){
            let res = Solution::remove_duplicates_v3(&mut test.nums);
            assert_eq!(res, test.result_len, "Test failed output: {:?}", test.nums);
        }


        Ok(())
    }
}