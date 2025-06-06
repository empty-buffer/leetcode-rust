struct Solution {}

impl Solution {
    fn bobble_sort(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() - 1 {
            let mut swapped = false;

            for j in 0..nums.len() - 1 - i {
                if nums[j] > nums[j + 1] {
                    nums.swap(j, j + 1);
                    swapped = true
                }
            }

            if !swapped {
                break;
            }
        }

        nums
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::Solution;

    pub type Result<T> = core::result::Result<T, Error>;
    pub type Error = Box<dyn std::error::Error>;

    #[test]
    fn test_bubble_sort() -> Result<()> {
        let nums: Vec<i32> = vec![8, 1, 3, 12, 5, 6];
        let res = Solution::bobble_sort(nums);
        // let rescult = find_majority_v1(nums.clone());
        assert_eq!(
            res,
            vec![1, 3, 5, 6, 8, 12],
            "Test failed with input: {:?}",
            res
        );
        Ok(())
    }
}
