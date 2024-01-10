fn main() {
    let nums: Vec<i32> = vec![1, 2, 1, 2, 1, 2, 2];
    println!("{}", find_majority_v1(nums))
}

fn find_majority_v1(nums: Vec<i32>) -> i32 {
    let mut majority = nums[0];
    let mut majority_frequency = 1;

    for i in 1..nums.len() {
        if majority_frequency == 0 {
            majority = nums[i];
        } else {
            if nums[i] == majority {
                majority_frequency += 1;
            } else {
                majority_frequency -= 1;
            }
        }
    }

    majority
}

#[cfg(test)]
mod tests {
    use crate::find_majority_v1;

    pub type Result<T> = core::result::Result<T, Error>;
    pub type Error = Box<dyn std::error::Error>;

    #[test]
    fn test_find_majority() -> Result<()> {
        let nums: Vec<i32> = vec![1, 2, 1, 2, 1, 2, 2];
        let result = find_majority_v1(nums.clone());
        assert_eq!(result, 2, "Test failed with input: {:?}", nums);

        let nums: Vec<i32> = vec![3, 3, 4, 2, 4, 4, 2, 4, 4];
        let result = find_majority_v1(nums.clone());
        assert_eq!(result, 4, "Test failed with input: {:?}", nums);

        let nums: Vec<i32> = vec![5, 5, 5, 0, 0, 0, 0];
        let result = find_majority_v1(nums.clone());
        assert_eq!(result, 0, "Test failed with input: {:?}", nums);

        let nums: Vec<i32> = vec![6];
        let result = find_majority_v1(nums.clone());
        assert_eq!(result, 6, "Test failed with input: {:?}", nums);
        Ok(())
    }
}