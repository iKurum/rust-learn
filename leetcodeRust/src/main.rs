mod hard;
mod junior_algorithm;
mod medium;
mod simple;

fn main() {
  let v = "RLRRLLRLRL".to_string();
  println!("{:?}", simple::c1221::test(v));
}
