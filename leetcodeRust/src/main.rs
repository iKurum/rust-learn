mod hard;
mod junior_algorithm;
mod medium;
mod simple;

fn main() {
  let nums: Vec<i32> = vec![1, 2, 3, 1];
  let result = junior_algorithm::vector::four::test(nums);
  println!("{:?}", result,);
}
