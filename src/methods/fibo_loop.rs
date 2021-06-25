pub mod fibo {
  #[allow(dead_code)]
  pub fn run(num: i32) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    let mut s = (0, 1);
    for i in 0..num {
      if i < 2 {
        v.push(i);
      } else {
        s = (s.1, s.0 + s.1);
        v.push(s.1);
      }
    }

    v
  }
}
