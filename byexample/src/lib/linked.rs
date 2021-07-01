#![allow(dead_code)]

#[derive(Debug)]
pub enum List {
  Cons(u32, Box<List>),
  Nil,
}

use List::*;
impl List {
  pub fn new() -> List {
    Nil
  }

  pub fn prepend(self, elem: u32) -> List {
    Cons(elem, Box::new(self))
  }

  pub fn len(&self) -> u32 {
    match *self {
      // 不能得到 tail 的所有权，因为 `self` 是借用的；
      // 因此使用一个对 tail 的引用
      Cons(_, ref tail) => 1 + tail.len(),
      Nil => 0,
    }
  }

  pub fn stringify(&self) -> String {
    match *self {
      Cons(head, ref tail) => {
        // `format!` 和 `print!` 类似，但返回的是一个堆分配的字符串
        format!("{}, {}", head, tail.stringify())
      }
      Nil => {
        format!("Nil")
      }
    }
  }
}
