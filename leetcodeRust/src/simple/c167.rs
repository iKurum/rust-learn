#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 给定一个已按照 **升序排列**  的整数数组 numbers ，请你从数组中找出两个数满足相加之和等于目标数 target 。
  ///
  /// 函数应该以长度为 2 的整数数组的形式返回这两个数的下标值。numbers 的下标 **从 1 开始计数** ，所以答案数组应当满足 1 <= answer[0] < answer[1] <= numbers.length 。
  ///
  ///你可以假设每个输入只对应唯一的答案，而且你不可以重复使用相同的元素。
  ///
  /// 提示：
  ///```
  /// 2 <= numbers.length <= 3 * 10^4
  /// -1000 <= numbers[i] <= 1000
  /// numbers 按 递增顺序 排列
  /// -1000 <= target <= 1000
  /// 仅存在一个有效答案
  ///```
  fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut scores: HashMap<i32, i32> = HashMap::new();
    let mut vec: Vec<i32> = Vec::new();

    for i in 0..numbers.len() {
      match scores.get(&(numbers[i])) {
        Some(&j) => {
          vec.push(j);
          vec.push((i + 1) as i32);
        }
        None => {
          scores.insert(target - numbers[i], (i + 1) as i32);
        }
      }
    }

    vec
  }
}

pub fn test(numbers: Vec<i32>, target: i32) -> Vec<i32> {
  Solution::two_sum(numbers, target)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn two_sum() {
    assert_eq!(vec![1, 2], Solution::two_sum(vec![2, 7, 11, 15], 9));
  }
}
