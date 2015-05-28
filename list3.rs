use std::fmt;
use List::{Cons, Nil};
use std::sync::Arc;
 
enum List<T> {
  Cons(T, Arc<List<T>>),
  Nil
}
 
impl<T> fmt::Display for List<T> where T : fmt::Display {
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
 
fn fold_right<T, U>(list: &List<T>, result: U, f: &Fn(T, U) -> U) -> U where T: Copy, T: Sized {
  match *list {
    Cons(ref head, ref tail) => 
      f(*head, fold_right(tail, result, f)),
    Nil => result
  }
}
 
fn new_list<T>(args: Vec<T>) -> List<T> where T: Copy {
  let mut vec = args;
  let mut x: List<T> = Nil;
  while vec.len() > 0 {
    match vec.pop() {
      Some(e) => x = Cons(e, Arc::new(x)),
      None => {}
    }
  }
  x
}
 
fn main() {
  let list = new_list(vec![1, 2, 3, 4, 5]);
  let dup_list = fold_right(&list, Nil, &|x, y| Cons(x, Arc::new(y)));
  println!("duplication of {list1} = {list2}", list1=list, list2=dup_list);
}
