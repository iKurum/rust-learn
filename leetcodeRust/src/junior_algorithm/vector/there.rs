#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 给定一个数组，将数组中的元素向右移动 k 个位置，其中 k 是非负数。
  pub fn rotate(nums: &mut Vec<i32>, k: i32) -> Vec<i32> {
    let k = (k % nums.len() as i32) as i32;
    let n = (nums.len() - 1) as i32;
    nums.reverse();
    reverse(nums, 0, k - 1);
    reverse(nums, k, n);

    nums.clone()
  }
}

fn reverse(nums: &mut Vec<i32>, mut start: i32, mut end: i32) {
  while start < end {
    let temp: i32;
    temp = nums[start as usize];
    nums[start as usize] = nums[end as usize];
    nums[end as usize] = temp;
    start += 1;
    end -= 1;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn rotate() {
    assert_eq!(
      vec![5, 6, 7, 1, 2, 3, 4],
      Solution::rotate(&mut vec![1, 2, 3, 4, 5, 6, 7], 3)
    )
  }
}
