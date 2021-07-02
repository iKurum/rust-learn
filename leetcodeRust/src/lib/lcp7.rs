#![allow(dead_code)]

struct Solution;

impl Solution {
  fn num_ways(n: i32, relation: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut dp: Vec<i32> = [0].repeat(n as usize);
    dp[0] = 1;

    for _ in 0..k as usize {
      let mut next: Vec<i32> = [0].repeat(n as usize);
      for v in relation.iter() {
        if let Some(u) = v.get(0) {
          if let Some(n) = v.get(1) {
            next[*n as usize] += dp[*u as usize];
          }
        }
      }
      dp = next;
    }

    dp[(n - 1) as usize]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn lcp7() {
    let relation = vec![
      vec![0, 2],
      vec![2, 1],
      vec![3, 4],
      vec![2, 3],
      vec![1, 4],
      vec![2, 0],
      vec![0, 4],
    ];
    assert_eq!(3, Solution::num_ways(5, relation, 3));
  }
}
