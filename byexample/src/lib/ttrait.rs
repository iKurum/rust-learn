#![allow(dead_code)]

pub struct Empty;
pub struct Null;

trait DoubleDrop<T> {
  fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
  fn double_drop(self, _: T) {}
}

pub fn dd<T, U>(e: T, n: U) {
  e.double_drop(n)
}

pub struct Container(pub i32, pub i32);

pub trait Contains {
  // 关联类型
  type A;
  type B;

  pub fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
  fn first(&self) -> i32;
  fn last(&self) -> i32;
}

impl Contains for Container {
  type A = i32;
  type B = i32;

  fn contains(&self, number_1: &Self::A, number_2: &Self::B) -> bool {
    (&self.0 == number_1) && (&self.1 == number_2)
  }

  fn first(&self) -> i32 {
    self.0
  }

  fn last(&self) -> i32 {
    self.1
  }
}

pub fn difference<C: Contains>(container: &C) -> i32 {
  container.last() - container.first()
}
