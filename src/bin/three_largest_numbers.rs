fn main() {
    let mut nums: Vec<i32> = vec![141, -1, -5, 16, 23, -51, 55, 99, -33, 43, 34];

    nums.sort();

    // println!("{:?}", &nums);
    println!("{:?}", &nums[nums.len() - 3..]);
    println!("sort solution: {:?}", Solution::sort(nums.clone()));
    println!("get, saturating: {:?}", Solution::get(nums))
}

struct Solution {}

impl Solution {
    fn sort(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();

        let last_three = nums.iter().rev().take(3).rev().copied().collect();
        last_three
    }

    fn get(nums: Vec<i32>) -> Vec<i32> {
        if let Some(last_three) = nums.get(nums.len().saturating_sub(3)..) {
            let result: Vec<i32> = last_three.to_vec();
            return result;
        }

        Vec::new()
    }
}
