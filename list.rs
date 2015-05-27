//use core::fmt::Display;
use std::fmt;
use List::{Cons, Nil};

enum List<T> {
  Cons(T, Box<List<T>>),
  Nil
}

impl<T> fmt::Display for List<T> where T : fmt::Display {//TがDisplay traitを実装していることを保証したい
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Cons(ref head, ref tail) => 
        {
          write!(f, "{head} ::", head = head);
          write!(f, "{tail}", tail = tail)
        }
      Nil => write!(f, "Nil")
    }
  }
}

fn main() {
  let list = Cons("A", Box::new(Cons("B", Box::new(Cons("C", Box::new(Nil))))));
  println!("Hello, {rust}", rust=list);
}
