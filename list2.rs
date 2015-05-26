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
          write!(f, "{head} :: ", head = head);
          write!(f, "{tail} ", tail = tail)
        }
      Nil => write!(f, "Nil")
    }
  }
}

fn new_list<T>(args: Vec<T>) -> Box<List<T>> where T: Copy {
  let mut vec = args;
  let mut x: Box<List<T>> = Box::new(Nil);
  while vec.len() > 0 {
    match vec.pop() {
      Some(e) => x = Box::new(Cons(e, x)),
      None => {}
    }
  }
  x
}

fn main() {
  let list = new_list(vec!["A", "B", "C"]);
  println!("Hello, {rust}", rust=list);
}
