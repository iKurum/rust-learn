mod hard;
mod junior_algorithm;
mod medium;
mod simple;

fn main() {
  let mut nums: Vec<i32> = vec![];
  let result = junior_algorithm::vector::one::test(&mut nums);
  println!(
    "{:?}, {:?}",
    result,
    nums
  );
}
