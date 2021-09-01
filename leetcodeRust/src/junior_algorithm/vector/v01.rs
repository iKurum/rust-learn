#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 给你一个有序数组 nums ，请你 **原地** 删除重复出现的元素，使每个元素 **只出现一次** ，返回删除后数组的新长度。
  ///
  ///不要使用额外的数组空间，你必须在 **原地修改输入数组** 并在使用 O(1) 额外空间的条件下完成。
  ///
  /// 提示：
  /// ```
  ///   0 <= nums.length <= 3 * 10^4
  ///   -10^4 <= nums[i] <= 10^4
  ///   nums 已按升序排列
  /// ```
  fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 {
      return 0;
    }
    let (mut l, mut r) = (0, 1);
    while r < nums.len() {
      println!("l {}-{}", l, nums[l as usize]);
      println!("r {}-{}", r, nums[r as usize]);

      if nums[l as usize] != nums[r as usize] {
        nums[(l + 1) as usize] = nums[r as usize];
        println!("{} - {}", l + 1, nums[(l + 1) as usize]);

        l += 1;
        r += 1;
      } else {
        r += 1;
      }
    }

    l + 1
  }
}

pub fn test(nums: &mut Vec<i32>) -> i32 {
  Solution::remove_duplicates(nums)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn remove_duplicates() {
    let mut nums = vec![1, 1, 2];
    let size = Solution::remove_duplicates(&mut nums);
    assert_eq!(vec![1, 2, 2], nums);
    assert_eq!(2, size);
  }
}
