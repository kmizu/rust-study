//use core::fmt::Display;
use std::fmt;
use List::{Cons, Nil};

enum List<T> {
  Cons(T, Box<List<T>>),
  Nil
}

impl<T> fmt::Display for List<T : Display> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Cons(ref value, ref tail) => write!(f, "{v} :: Nil", v = value),
      Nil => write!(f, "Nil")
    }
  }
}

fn main() {
  let list = Cons("A", Box::new(Cons("B", Box::new(Nil))));
  println!("Hello, :?{rust}", rust=list);
}
