enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

// derf trait, treat smart pointer like regular reference
use std::ops::Deref;
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn use_mybox() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // rust actually calls assert_eq!(5, *(y.deref()))

    let m = MyBox::new(String::from("Vitalic"));
    // automatic deref coercion: &MyBox<String> -> &String -> &str
    hello(&m); // hello(&(*)m[..]);
}

fn hello(name: &str) {
    println!("hello {}", name);
}

// drop traitasa

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("{}", self.data);
    }
}

fn drop_before_end() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };

    drop(c); // c.drop() is not allowed and rust will clean memory automatically

    println!("c dropped before the end of main")
}
