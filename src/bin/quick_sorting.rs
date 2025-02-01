fn main() {
    let mut test = vec![64, 34, 25, 12, 22, 11, 90];
    quick_sort(&mut test);

    let hist = 0;
    println!("{:?}", test)
}

fn quick_sort(arr: &mut Vec<i32>) {
    if arr.is_empty() {
        return;
    }

    helper(arr, 0, arr.len() - 1);
}

fn helper(arr: &mut Vec<i32>, low: usize, high: usize) {
    if low < high {
        let index = partition(arr, low, high);

        if index > 0 {
            helper(arr, low, index - 1);
        }
        helper(arr, index + 1, high);
    }
}

fn partition(arr: &mut Vec<i32>, low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i = low as i32 - 1;

    for j in low..high {
        if arr[j] <= pivot {
            i += 1;
            if i as usize != j {
                arr.swap(i as usize, j);
            }
        }
    }

    if (i + 1) as usize != high {
        arr.swap((i + 1) as usize, high);
    }

    (i + 1) as usize
}
