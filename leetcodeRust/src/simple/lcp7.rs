#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 小朋友 A 在和 ta 的小伙伴们玩传信息游戏，游戏规则如下：
  /// 
  /// - 有 n 名玩家，所有玩家编号分别为 0 ～ n-1，其中小朋友 A 的编号为 0
  /// - 每个玩家都有固定的若干个可传信息的其他玩家（也可能没有）。传信息的关系是单向的（比如 A 可以向 B 传信息，但 B 不能向 A 传信息）
  /// - 每轮信息必须需要传递给另一个人，且信息可重复经过同一个人
  /// 
  /// 给定总玩家数 n，以及按 [玩家编号,对应可传递玩家编号] 关系组成的二维数组 relation。返回信息从小 A (编号 0 ) 经过 k 轮传递到编号为 n-1 的小伙伴处的方案数；若不能到达，返回 0。
  /// 
  /// 限制：
  /// 
  /// ```
  ///   2 <= n <= 10
  ///   1 <= k <= 5
  ///   1 <= relation.length <= 90, 且 relation[i].length == 2
  ///   0 <= relation[i][0],relation[i][1] < n 且 relation[i][0] != relation[i][1]
  /// ```
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
