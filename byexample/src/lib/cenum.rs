#![allow(dead_code)]

// 隐式辨别值，从 0 开始
enum Number {
  Zero,
  One,
  Two,
}

// 显示辨别值
enum Color {
  Red = 0xff0000,
  Green = 0x00ff00,
  Blue = 0x0000ff,
}

pub fn asi32() {
  println!("{}", Number::Zero as i32);
  println!("{:06x}", Color::Blue as i32);
}