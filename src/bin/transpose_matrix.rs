/*
[0 1 2]
[3 4 5]
[6 7 8]
*/

fn main() {
    let mut matrix = Vec::new();
    matrix.push(vec![0, 1, 2]);
    matrix.push(vec![3, 4, 5]);
    matrix.push(vec![6, 7, 8]);

    println!("-----");
    for row in Solution::version_0(matrix.clone()) {
        println!("{:?}", row)
    }

    println!("-----");
    for row in Solution::version_1(matrix) {
        println!("{:?}", row)
    }
}

struct Solution {}

impl Solution {
    fn version_0(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut transpose_matrix: Vec<Vec<i32>> = Vec::new();
        for col in 0..matrix[0].len() {
            let mut new_row: Vec<i32> = Vec::new();

            for row in 0..matrix.len() {
                new_row.push(matrix[row][col]);
            }

            transpose_matrix.push(new_row);
        }

        transpose_matrix
    }

    fn version_1(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for i in 0..matrix.len() {
            for j in i + 1..matrix[i].len() {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp
            }
        }

        matrix
    }

    // fn version_mem_swap(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    //     for i in 0..matrix.len() {
    //         for j in i + 1..matrix[i].len() {
    //             mem::swap(&mut matrix[i][j], &mut matrix[j][i]);
    //         }
    //     }

    //     matrix
    // }
}
