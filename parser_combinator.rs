use std::ops::Add;
use std::ops::BitOr;
use std::sync::Arc;

struct Parser<T> {
    body: Arc<Fn(Arc<String>) -> Option<(T, Arc<String>)>>
}

fn parser_of<A>(func: Arc<Fn(Arc<String>) -> Option<(A, Arc<String>)>>) -> Parser<A> {
    Parser{ body: func }
}

fn literal(value: Arc<String>) -> Parser<String> {
    parser_of(
        Arc::new(move |input: Arc<String>| {
            if (*input.clone()).starts_with(&*value.clone()) {
                Some((value.clone().to_string(), Arc::new(input[(value.clone().len())..input.len()].to_string())))
            } else {
                None
            }
        })
    )
}

impl<T: 'static, U: 'static> Add<Parser<U>> for Parser<T> {
    type Output  = Parser<(T, U)>;
    fn add(self, other: Parser<U>) -> Parser<(T, U)> {
        let result: Parser<(T, U)> = parser_of(
            Arc::new(move |input: Arc<String>| {
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
            Arc::new(move |input: Arc<String>| {
                match (self.body)(input.clone()) {
                    Some((value1, rest1)) => 
                        Some((value1, rest1)),
                    None => 
                        match (other.body)(input.clone()) {
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
    let v = literal(Arc::new("H".to_string())) + literal(Arc::new("e".to_string()));
    match (v.body)(Arc::new("Hello".to_string())) {
        Some(((v1, v2), r)) => {
            println!("{}", v1.to_string());
            println!("{}", v2.to_string());
            println!("{}", r.to_string());
        }
        None => {
        }
    }
}
