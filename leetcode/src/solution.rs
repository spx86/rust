struct Solution;
impl Solution {
    /*
        //时间复杂度: O(n^2)
        //kong空间复杂度: O(n^2)

    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
         let n = matrix.len();
         *matrix = (0..n)
             .map(|j| (0..n).rev().map(|i| matrix[i][j]).collect::<Vec<_>>())
             .collect::<Vec<_>>();
    }
    */

    // 时间复杂度: O(n^2)
    // 空间复杂度: O(1)
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        //转置
        for i in 0..n {
            for j in i + 1..n {
                matrix[i][j] ^= matrix[j][i];
                matrix[j][i] ^= matrix[i][j];
                matrix[i][j] ^= matrix[j][i];
            }
        }
        //反转每一行
        for row in matrix.iter_mut() {
            row.reverse();
        }
    }

    // pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    //     let m = matrix.len();
    //     let n = matrix[0].len();
    //     let mut first_row_has_zero = false;
    //     let mut first_col_has_zero = false;

    //     // 检查第一行是否有0
    //     for j in 0..n {
    //         if matrix[0][j] == 0 {
    //             first_row_has_zero = true;
    //             break;
    //         }
    //     }

    //     // 检查第一列是否有0
    //     for i in 0..m {
    //         if matrix[i][0] == 0 {
    //             first_col_has_zero = true;
    //             break;
    //         }
    //     }

    //     // 使用第一行和第一列记录0的位置
    //     for i in 1..m {
    //         for j in 1..n {
    //             if matrix[i][j] == 0 {
    //                 matrix[i][0] = 0; // 标记行
    //                 matrix[0][j] = 0; // 标记列
    //             }
    //         }
    //     }

    //     // 根据标记清零内部区域
    //     for i in 1..m {
    //         for j in 1..n {
    //             if matrix[i][0] == 0 || matrix[0][j] == 0 {
    //                 matrix[i][j] = 0;
    //             }
    //         }
    //     }

    //     // 清零第一行
    //     if first_row_has_zero {
    //         for j in 0..n {
    //             matrix[0][j] = 0;
    //         }
    //     }

    //     // 清零第一列
    //     if first_col_has_zero {
    //         for i in 0..m {
    //             matrix[i][0] = 0;
    //         }
    //     }
    // }

    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let mut x: Vec<(usize, usize)> = Vec::new();
        (0..m).for_each(|i| {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 0 {
                    x.push((i, j));
                }
            }
        });

        for (i, j) in x {
            for k in 0..matrix[0].len() {
                matrix[i][k] = 0;
            }
            for k in 0..m {
                matrix[k][j] = 0;
            }
        }
    }

    // pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
    //     if mat.is_empty() || mat[0].is_empty() {
    //         return Vec::new();
    //     }
    //     let m = mat.len();
    //     let n = mat[0].len();
    //     let mut result = Vec::with_capacity(m * n);
    //     for i in 0..m + n - 1 {
    //         if i % 2 == 0 {
    //             // 从下往上
    //             let start_row = if i < m { i } else { m - 1 };
    //             let start_col = if i < m { 0 } else { i - m + 1 };
    //             let mut row = start_row;
    //             let mut col = start_col;
    //             while col < n {
    //                 result.push(mat[row][col]);
    //                 if row == 0 {
    //                     break;
    //                 } else {
    //                     row -= 1;
    //                     col += 1;
    //                 }
    //             }
    //         } else {
    //             // 从上往下
    //             let start_row = if i < n { 0 } else { i - n + 1 };
    //             let start_col = if i < n { i } else { n - 1 };
    //             let mut row = start_row;
    //             let mut col = start_col;
    //             while row < m {
    //                 result.push(mat[row][col]);
    //                 if col == 0 {
    //                     break;
    //                 } else {
    //                     row += 1;
    //                     col -= 1;
    //                 }
    //             }
    //         }
    //     }
    //     result
    // }

    pub fn find_diagonal_order_fp(mat: &[Vec<i32>]) -> Vec<i32> {
        if mat.is_empty() || mat[0].is_empty() {
            return Vec::new();
        }

        let m = mat.len();
        let n = mat[0].len();

        (0..(m + n - 1))
            .flat_map(|i| {
                let (start_row, start_col, step) = if i % 2 == 0 {
                    (i.min(m - 1), i.saturating_sub(m - 1), (-1, 1))
                } else {
                    (i.saturating_sub(n - 1), i.min(n - 1), (1, -1))
                };

                let mut current = (start_row as isize, start_col as isize);
                std::iter::from_fn(move || {
                    if current.0 >= 0 && current.1 >= 0 {
                        let (r, c) = (current.0 as usize, current.1 as usize);
                        if r < m && c < n {
                            let val = mat[r][c];
                            current = (current.0 + step.0, current.1 + step.1);
                            Some(val)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_rotate() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3],]);
    }

    #[test]
    fn test_set_zeroes() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);
    }

    #[test]
    fn test_find_diagonal_order() {
        let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = Solution::find_diagonal_order_fp(&mat);
        assert_eq!(result, vec![1, 2, 4, 7, 5, 3, 6, 8, 9]);
    }
}
