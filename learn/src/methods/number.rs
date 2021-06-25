#[allow(dead_code)]
pub mod num {
  use std::collections::HashMap;

  #[derive(Debug)]
  pub struct Num {
    old_vec: Vec<i32>,
    new_vec: Vec<i32>,
    map: HashMap<i32, i32>,
  }

  impl Num {
    pub fn new(v: Vec<i32>) -> Num {
      let mut map = HashMap::new();
      let mut vec = v.clone();
      let vec_ = v;

      for n in vec.iter() {
        let count = map.entry(*n).or_insert(0);
        *count += 1;
      }

      vec.sort_by(|a, b| a.cmp(b));
      Num {
        old_vec: vec_,
        new_vec: vec,
        map,
      }
    }
  }

  impl Num {
    pub fn average(&self) -> i32 {
      let mut sum = 0;
      let l = self.new_vec.len();
      for n in self.new_vec.iter() {
        sum += n;
      }

      sum / l as i32
    }

    pub fn mid(&self) -> i32 {
      let l = self.new_vec.len() / 2;
      if self.new_vec.len() % 2 == 1 {
        self.new_vec[l]
      } else {
        (self.new_vec[l] + self.new_vec[l + 1]) / 2
      }
    }

    pub fn more(&self) -> i32 {
      let mut m = 0;
      let mut r = 0;
      for (k, v) in &self.map {
        if *v > m {
          m = *v;
          r = *k;
        }
      }

      r
    }
  }
}
