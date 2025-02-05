fn main() {
    let n = 5;

    println!("{}", test(n))
}

fn test(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }

    // Create variables to store previous two values
    let mut prev1 = 1; // represents i-1
    let mut prev2 = 1; // represents i-2
    let mut current = 0;

    // Calculate ways for each step from 2 to n
    for _ in 2..=n {
        current = prev1 + prev2;
        prev2 = prev1;
        prev1 = current;
    }

    current
}
// TODO
// {34, 38, 55, 1} {4, 7, 19, 22}
