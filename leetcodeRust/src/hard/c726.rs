use regex::{Regex};

struct Solution;

impl Solution {
  
    pub fn count_of_atoms(formula: String) -> String {
      let r = Regex::new(r"[A_Z]").expect("正则错误");
      println!("{:?}", r.split(&formula).collect::<String>());

      formula
    }
}

pub fn test(formula: String) -> String {
  Solution::count_of_atoms(formula)
}