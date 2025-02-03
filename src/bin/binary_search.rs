use std::fmt::Display;

fn main() {
    let nums = vec![0, 21, 44, 54, 65, 77, 78, 88, 90, 100];

    let len = nums.len();

    let result = helper(nums, 1, 0, len - 1);

    println!("{}", result)
}

struct Search {
    target: Option<i32>,
    closest: Option<i32>,
}

impl Display for Search {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Target: {:?}, Closest: {:?}", self.target, self.closest)
    }
}

fn helper(nums: Vec<i32>, target: i32, low: usize, high: usize) -> Search {
    if low > high {
        if low <= nums.len() - 1 {
            let mut l_closest = nums[low] - target;
            let mut h_closest = nums[high] - target;

            l_closest = l_closest.max(-l_closest);
            h_closest = h_closest.max(-h_closest);

            if l_closest < h_closest {
                return Search {
                    target: None,
                    closest: Some(nums[low]),
                };
            }
        }

        return Search {
            target: None,
            closest: Some(nums[high]),
        };
    }

    let middle = (low + high) / 2;

    if nums[middle] == target {
        return Search {
            target: Some(nums[middle]),
            closest: None,
        };
    } else if nums[middle] > target {
        return helper(nums, target, low, middle - 1);
    } else {
        return helper(nums, target, middle + 1, high);
    }
}
