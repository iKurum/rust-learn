#![allow(dead_code)]

struct Solution;

impl Solution {
    fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
      let n = nums.len();
      let mut negative: i32 = -1;
      for i in 0..n {
        if nums[i] < 0 {
          negative = i as i32;
        } else {
          break;
        }
      }

      let mut ans: Vec<i32> = Vec::new();
      let (mut i, mut j) = (negative, (negative + 1) as usize);
      while i >= 0 || j < n {
        if j == n {
          ans.push(nums[i as usize] * nums[i as usize]);
          i -= 1;
        } else if i < 0 {
          ans.push(nums[j] * nums[j]);
          j += 1;
        } else if nums[i as usize] * nums[i as usize] < nums[j] * nums[j] {
          ans.push(nums[i as usize] * nums[i as usize]);
          i -= 1;
        } else {
          ans.push(nums[j] * nums[j]);
          j += 1;
        }
      }

      ans
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sorted_squares() {
      assert_eq!(vec![0,1,9,16,100], Solution::sorted_squares(vec![-4,-1,0,3,10]));
  }
}