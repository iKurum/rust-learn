mod methods;
#[allow(unused_imports)]
use crate::methods::{cp, fibo, first_word, guess, linked_list, pig_latin, Num, User};

fn main() {
    // guess::run();
    // println!("{:?}", fibo::run(10));

    // let s = String::from("Hello world");
    // println!("The first word: {}", first_word(&s[..]));
    // let s = "Hello world";
    // println!("The first word: {}", first_word(s));

    // let user = User::new("cyan", "cyan@ikurum.cn");
    // println!("{:#?}", user);

    // use std::collections::HashMap;
    // let text = "hello world wonderful world";
    // let mut map = HashMap::new();
    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }
    // println!("{:?}", map);

    // let v = Num::new(vec![1, 2, 3, 3, 5, 1, 1, 1, 1]);
    // println!("{:?}", v);
    // println!("this average is: {}", v.average());
    // println!("this mid is: {}", v.mid());
    // println!("this more is: {}", v.more());

    // println!("{}: {}", "apple", pig_latin("apple"));
    // println!("{}: {}", "First", pig_latin("First"));

    // cp::run();

    let mut list = linked_list::List::new(0);
    // 追加一些元素
    list.push(1);
    list.push(2);
    list.push(3);
    list.unshift(4);
    // 显示链表的最后状态
    println!("linked list has length: {}", list.len());
    println!("{:?}", list.stringify());
}
