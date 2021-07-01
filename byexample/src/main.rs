mod lib;
#[allow(unused_imports)]
use lib::{cenum::asi32, List};
use lib::ttrait::{self, Container};

fn main() {
    // asi32();

    // let mut list = List::new();
    // println!("list 0: {:?}", list);
    // list = list.prepend(1);
    // println!("list 1: {:?}", list);
    // list = list.prepend(2);
    // println!("list 2: {:?}", list);
    // println!("list len: {}", list.len());
    // println!("{}", list.stringify());

    // let empty = ttrait::Empty;
    // let null = ttrait::Null;
    // ttrait::dd(empty, null);

    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!(
        "Does container contain {} and {}: {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    println!("The difference is: {}", ttrait::difference(&container));
}
