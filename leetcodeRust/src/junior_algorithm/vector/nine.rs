#![allow(dead_code)]

struct Solution;

impl Solution {
  fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut map: HashMap<i32, usize> = HashMap::new();
    let mut result: Vec<i32> = Vec::new();
    for i in 0..nums.len() {
      let r = target - nums[i];
      if let Some(&l) = map.get(&r) {
        result.push(i as i32);
        result.push(l as i32);
      } else {
        map.insert(r, i);
      }
    }

    result
  }
}
