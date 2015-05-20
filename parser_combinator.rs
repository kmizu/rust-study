use std::ops::Add;
use std::ops::BitOr;

struct Parser<T> {
  body: Box<Fn(String) -> Option<(T, String)>>
}

fn parser_of<A>(func: Box<Fn(String) -> Option<(A, String)>>) -> Parser<A> {
  Parser{ body: func }
}

impl<T: 'static, U: 'static> Add<Parser<U>> for Parser<T> {
  type Output  = Parser<(T, U)>;
  fn add(self, other: Parser<U>) -> Parser<(T, U)> {
    let result: Parser<(T, U)> = parser_of(
      Box::new(move |input: String| -> Option<((T, U), String)> {
        match (self.body)(input) {
          Some((value1, rest1)) => 
            match (other.body)(rest1) {
              Some((value2, rest2)) =>
                Some(((value1, value2), rest2)),
              None => None
            },
          None => None
        }
      })
    );
    result
  }
}

impl<T: 'static> BitOr<Parser<T>> for Parser<T> {
  type Output  = Parser<T>;
  fn bitor(self, other: Parser<T>) -> Parser<T> {
    let result: Parser<T> = parser_of(
      Box::new(move |input: String| -> Option<(T, String)> {
        match (self.body)(input) {
          Some((value1, rest1)) => 
            Some((value1, rest1)),
          None => 
            match (other.body)(input) {
              Some((value2, rest2)) => Some((value2, rest2)),
              None => None
            }
        }
      })
    );
    result
  }
}

fn main() {
  println!("Parser Combinator!");
}
