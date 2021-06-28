use spointer::List::{Cons, Nil};
use spointer::{CustomSmartPointer, MyBox, Node};
use std::cell::RefCell;
use std::mem::drop;
use std::rc::{Rc, Weak};

fn main() {
    // cons rc
    // let a = Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))));
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    println!("count after creating a = {}", Rc::strong_count(&a));
    println!("a before = {:?}", a);

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    // RefCell 可变借用
    *value.borrow_mut() += 10;

    {
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
        println!("c after = {:?}", c);
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    println!("b after = {:?}", b);

    // deref
    println!("==========================");
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // drop
    println!("==========================");
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    println!("c CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("d CustomSmartPointers created.");

    // Weak
    println!("==========================");
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
    // println!("branch parent = {:?}", branch.parent.borrow().upgrade());
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    println!("==========================");
}
