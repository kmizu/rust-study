use lists::{contains, new_list};
 
mod lists {
  use std::fmt;
  use lists::List::{Cons, Nil};
  use std::sync::Arc;

  #[derive(PartialEq, Clone)]
  pub enum List<T> {
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

  impl<T> List<T> {
    #[allow(dead_code)]
    pub fn flat_map<U>(&self, f: &Fn(T) -> List<U>) -> List<U> where T: Copy, T: Sized, T: Copy, T: PartialEq, U: Copy, U: Sized, U: PartialEq {
      match *self {
        Cons(ref head, ref tail) => concat(&f(*head), tail.flat_map(f)),
        Nil => Nil
      }
    }
  }

  #[allow(dead_code)]
  pub fn contains<T>(list: &List<T>, e: T) -> bool where T: PartialEq {
    match *list {
      Cons(ref head, ref tail) => if *head == e { true } else { contains(tail, e) },
      Nil => false
    }
  }

  #[allow(dead_code)]
  pub fn filter<T>(list: &List<T>, f: &Fn(T) -> bool) -> List<T> where T : Copy {
    fold_right(list, Nil, &|x, y| if f(x) { Cons(x, Arc::new(filter(&y, f))) } else { filter(&y, f) })
  }

  #[allow(dead_code)]
  pub fn concat<T>(list: &List<T>, b: List<T>) -> List<T> where T: Copy, T: Sized {
    fold_right(list, b, &|x, y| Cons(x, Arc::new(y)))
  }
 
  #[allow(dead_code)]
  pub fn fold_right<T, U>(list: &List<T>, result: U, f: &Fn(T, U) -> U) -> U where T: Copy, T: Sized {
    match *list {
      Cons(ref head, ref tail) => 
        f(*head, fold_right(tail, result, f)),
      Nil => result
    }
  }

  #[allow(dead_code)]
  pub fn fold_left<T, U>(list: &List<T>, result: U, f: &Fn(U, T) -> U) -> U where T: Copy, T: Sized {
    match *list {
      Cons(ref head, ref tail) => fold_left(tail, f(result, *head), f), 
      Nil => result
    }
  }
 
  #[allow(dead_code)]
  pub fn map<T, U>(list: &List<T>, f: &Fn(T) -> U) -> List<U> where T: Copy , U: Copy {
    fold_right(list, Nil, &|x, y| Cons(f(x), Arc::new(y)))
  }
   
  #[allow(dead_code)]
  pub fn new_list<T>(args: Vec<T>) -> List<T> where T: Copy {
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
}
 
fn main() {
  let list = new_list(vec![1, 2, 3, 4, 5]);
  println!("{list} contains 3 == {result}", list=list, result=contains(&list, 3));
  println!("{list} contains 6 == {result}", list=list, result=contains(&list, 6));
}
