use std::{env, error::Error, fs};

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: std::env::Args) -> Result<Self, &'static str> {
    if args.len() < 2 {
      return Err("必要参数错误/未传");
    }

    let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    let mut query = String::from("");
    let mut filename = String::from("");

    for item in args {
      if item.starts_with("f=") {
        let item = item.split("=").collect::<Vec<&str>>();
        match item.get(1) {
          Some(f) => filename = f.to_string(),
          None => continue,
        }
      } else if item.starts_with("q=") {
        let item = item.split("=").collect::<Vec<&str>>();
        match item.get(1) {
          Some(f) => query = f.to_string(),
          None => continue,
        }
      } else if item.starts_with(":") {
        match &item[..] {
          ":i" => {
            case_sensitive = true;
          }
          ":I" => {
            case_sensitive = false;
          }
          _ => continue,
        }
      }
    }

    if filename == "" {
      return Err("文件名参数错误/未传");
    }

    Ok(Self {
      query,
      filename,
      case_sensitive,
    })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;
  let results = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };

  for line in results {
    println!("{}", line);
  }

  Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents.lines().filter(|x| x.contains(query)).collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents
    .lines()
    .filter(|x| x.to_lowercase().contains(&query.to_lowercase()))
    .collect()
}
