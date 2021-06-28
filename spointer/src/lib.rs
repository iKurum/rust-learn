mod test;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub enum List {
  // Cons(i32, Box<List>),
  // Cons(i32, Rc<List>),
  Cons(Rc<RefCell<i32>>, Rc<List>),
  Nil,
}

// deref
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
  pub fn new(x: T) -> Self {
    Self(x)
  }
}

use std::ops::Deref;
impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &T {
    &self.0
  }
}

// drop
pub struct CustomSmartPointer {
  pub data: String,
}

impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("Dropping CustomSmartPointer with data `{}`!", self.data);
  }
}

// Weak<T>
#[derive(Debug)]
pub struct Node {
  pub value: i32,
  pub parent: RefCell<Weak<Node>>,
  pub children: RefCell<Vec<Rc<Node>>>,
}
