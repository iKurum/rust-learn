#[allow(dead_code)]
pub mod other {
  pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
        return &s[..i];
      }
    }

    s
  }

  #[derive(Debug)]
  pub struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
  }

  impl User {
    pub fn new(username: &str, email: &str) -> User {
      User {
        username: username.to_string(),
        email: email.to_string(),
        active: true,
        sign_in_count: 1,
      }
    }
  }

  pub fn pig_latin(str: &str) -> String {
    const VOWEL: &str = "aeiou";

    let o = str
      .chars()
      .into_iter()
      .map(|x| x.to_string())
      .collect::<Vec<_>>();

    if VOWEL.contains(&o[0]) {
      format!("{}-hay", str.to_string())
    } else {
      format!("{}-{}ay", o[1..].concat(), o[0])
    }
  }
}
