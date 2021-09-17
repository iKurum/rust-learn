#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 请你判断一个 9x9 的数独是否有效。只需要 根据以下规则 ，验证已经填入的数字是否有效即可。
  /// 1. 数字 1-9 在每一行只能出现一次。
  /// 2. 数字 1-9 在每一列只能出现一次。
  /// 3. 数字 1-9 在每一个以粗实线分隔的 3x3 宫内只能出现一次。
  /// 4. 数独部分空格内已填入了数字，空白格用 '.' 表示。
  ///
  /// 注意：
  ///
  /// 一个有效的数独（部分已被填充）不一定是可解的。
  ///
  /// 只需要根据以上规则，验证已经填入的数字是否有效即可。
  ///
  /// 提示：
  /// ```
  ///   board.length == 9
  ///   board[i].length == 9
  ///   board[i][j] 是一位数字或者 '.'
  /// ```
  fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut row = vec![vec![0; 9]; 9];
    let mut col = vec![vec![0; 9]; 9];
    let mut blk = vec![vec![0; 9]; 9];

    for i in 0..board.len() {
      for j in 0..board[i].len() {
        if board[i][j] != '.' {
          let num = (board[i][j] as u32 - '1' as u32) as usize;
          let blk_index = (i / 3 * 3 + j / 3) as usize;

          if row[i][num] == 1 || col[j][num] == 1 || blk[blk_index][num] == 1 {
            return false;
          } else {
            row[i][num] = 1;
            col[j][num] = 1;
            blk[blk_index][num] = 1;
          }
        }
      }
    }

    true
  }
}

pub fn test(board: Vec<Vec<char>>) -> bool {
  Solution::is_valid_sudoku(board)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn is_valid_sudoku() {
    assert_eq!(
      true,
      Solution::is_valid_sudoku(vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
      ])
    )
  }
}
