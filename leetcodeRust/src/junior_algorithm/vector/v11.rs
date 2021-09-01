#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 给定一个 n × n 的二维矩阵 matrix 表示一个图像。请你将图像顺时针旋转 90 度。
  ///
  /// 你必须在 **原地** 旋转图像，这意味着你需要直接修改输入的二维矩阵。**请不要** 使用另一个矩阵来旋转图像。
  ///
  /// 提示：
  ///  ```
  ///   matrix.length == n
  ///   matrix[i].length == n
  ///   1 <= n <= 20
  ///   -1000 <= matrix[i][j] <= 1000
  ///  ```
  fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let length = matrix.len();

    for i in 0..(length / 2) {
      let t = &matrix[i].clone();
      matrix[i] = (&matrix[length - i - 1]).to_vec();
      matrix[length - i - 1] = t.to_vec();
    }

    for i in 0..length {
      for j in (i + 1)..length {
        let t = matrix[i][j];
        matrix[i][j] = matrix[j][i];
        matrix[j][i] = t;
      }
    }
  }
}

pub fn test(matrix: &mut Vec<Vec<i32>>) {
  Solution::rotate(matrix);
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn rotate() {
    let mut v = vec![
      vec![5, 1, 9, 11],
      vec![2, 4, 8, 10],
      vec![13, 3, 6, 7],
      vec![15, 14, 12, 16],
    ];
    Solution::rotate(&mut v);
    assert_eq!(
      vec![
        vec![15, 13, 2, 5],
        vec![14, 3, 4, 1],
        vec![12, 6, 8, 9],
        vec![16, 7, 10, 11]
      ],
      v
    );
  }
}
