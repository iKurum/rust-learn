mod hard;
mod junior_algorithm;
mod medium;
mod simple;

fn main() {
  println!(
    "{:#?}",
    hard::c68::test(
      vec![
        "This".to_string(),
        "is".to_string(),
        "an".to_string(),
        "example".to_string(),
        "of".to_string(),
        "text".to_string(),
        "justification.".to_string()
      ],
      16
    )
  );
}
