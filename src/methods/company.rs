#[allow(dead_code)]
pub mod cp {
  use regex::Regex;
  use std::io::{self, Write};
  use std::{collections::HashMap, thread::sleep, time::Duration};

  trait Set {
    fn add(&mut self, i: Vec<&str>) -> Result<(), String>;

    fn list(&self, s: &str);
  }

  #[derive(Debug)]
  struct Cmap {
    mp: HashMap<String, Vec<String>>,
  }

  impl Set for Cmap {
    fn add(&mut self, i: Vec<&str>) -> Result<(), String> {
      if i.len() != 3 || i.get(1) != Some(&"to") {
        return Err(format!("Add 参数错误: {:?}, 详情请查看 help", i));
      }

      let v = self.mp.entry(i[2].to_string()).or_insert(Vec::new());
      v.push(i[0].to_string());

      print(&format!("Add {}...", i.join(" ")));
      sleep(Duration::from_secs(2));
      print("success\n");
      Ok(())
    }

    fn list(&self, s: &str) {
      if self.mp.len() != 0 {
        if s == "all" || s == "" {
          println!("{:#?}", self.mp);
        } else {
          match self.mp.get(s) {
            Some(part) => println!("{}: {:#?}", s, part),
            None => println!("没有部门：{}", s),
          }
        }
      } else {
        println!("还没有部门 ~~");
      }
    }
  }

  impl Cmap {
    fn init() -> Self {
      let mp = HashMap::new();

      Self { mp }
    }
  }

  fn print(str: &str) {
    print!("{}", str);
    io::stdout().flush().expect("Failed to flush stdout");
  }

  fn help() {
    println!("帮助菜单：");
    println!(
      "- add [name] to [part]  ...新增人员到部门\n
- list [all/part]       ...查看部门及人员\n
- clear                 ...清除记录\n
- exit                  ...退出\n
- help                  ...帮助菜单\n"
    );
  }

  pub fn run() {
    let mut map = Cmap::init();

    println!("部门人员管理");
    help();
    loop {
      print("> ");
      let mut i = String::new();
      io::stdin().read_line(&mut i).expect("Failed to read line");

      if i.trim() == "exit" {
        break;
      }

      let i = Regex::new(r"\s+")
        .unwrap()
        .split(i.trim())
        .collect::<Vec<&str>>();

      if let Some(&t) = i.get(1) {
        match i.get(0) {
          Some(&"add") => match map.add(i[1..].to_vec()) {
            Ok(_) => continue,
            Err(str) => {
              println!("Error: {}", str)
            }
          },
          Some(&"list") => map.list(t),
          _ => {
            println!("指令错误，请重新输入 ...");
          }
        }
      } else {
        match i.get(0) {
          Some(&"help") => help(),
          Some(&"list") => map.list(""),
          Some(&"clear") => {
            print!("\x1b[2J");
            print!("\x1b[H");
            println!("部门人员管理");
          }
          _ => continue,
        }
      }
    }
  }
}
