use cons::csp::CustomSmartPointer;
use cons::mybox::MyBox;
use std::mem;
use std::rc::Rc;

#[derive(Debug)]
enum BoxList {
    Cons(i32, Box<BoxList>),
    Nil,
}

#[derive(Debug)]
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

fn main() {
    let list = BoxList::Cons(
        1,
        Box::new(BoxList::Cons(
            2,
            Box::new(BoxList::Cons(3, Box::new(BoxList::Nil))),
        )),
    );

    println!("{:#?}", list);

    println!("-----");

    let x = 5;
    assert_eq!(5, x);
    println!("{}", x);
    let y = &x;
    assert_eq!(5, *y);
    println!("{}", *y);
    let z = MyBox::new(x);
    assert_eq!(5, *z);
    println!("{}", *z);
    let u = MyBox::new(&z);
    assert_eq!(5, ***u);
    println!("{}", ***u);
    let r = MyBox::new(*z);
    assert_eq!(5, *r);
    println!("{}", *r);
    let t = MyBox::new(z);
    assert_eq!(5, **t);
    println!("{}", **t);

    hello("Jane");

    let m = MyBox::new(String::from("John"));

    hello(&(*m));
    hello(&(*m)[1..3]);
    hello(&(*m)[..]);
    hello(&m[1..2]);
    hello(&m);

    println!("-----");

    let v1 = CustomSmartPointer::new(String::from("foo"));
    let v2 = CustomSmartPointer::new(String::from("bar"));

    println!("v1: {:?}", v1.data());
    mem::drop(v1);

    println!("v2: {:?}", v2.data());

    println!("-----");

    let a = Rc::new(RcList::Cons(
        5,
        Rc::new(RcList::Cons(10, Rc::new(RcList::Nil))),
    ));
    println!(
        "Reference count of 'a' after creating 'a': {}",
        Rc::strong_count(&a)
    );

    let b = RcList::Cons(3, Rc::clone(&a));
    println!(
        "Reference count of 'a' after creating 'b': {}",
        Rc::strong_count(&a)
    );

    {
        let c = RcList::Cons(4, Rc::clone(&a));
        println!(
            "Reference count of 'a' after creating 'c': {}",
            Rc::strong_count(&a)
        );
    }

    println!(
        "Reference count of 'a' after 'c' goes out of scope: {}",
        Rc::strong_count(&a)
    );
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
