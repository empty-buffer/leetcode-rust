fn main() {
    let nums = vec![-7, -5, -4, 3, 6, 8, 9];

    let len = nums.len();

    let result = run(nums, 0, len - 1);

    println!("{:?}", result);

    debug_assert_eq!(vec![8, 16, 25, 36, 49, 64, 81], result)
}

fn run(nums: Vec<i32>, mut s: usize, mut e: usize) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    for _i in 0..nums.len() {
        if nums[s].abs() > nums[e].abs() {
            result.insert(0, nums[s] * nums[s]);
            s += 1;
        } else {
            result.insert(0, nums[e] * nums[e]);
            e -= 1;
        }
    }

    result
}
