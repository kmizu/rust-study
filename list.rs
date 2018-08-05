use lists::new_list;
use lists::List::{Cons, Nil};

mod lists {
    use lists::List::{Cons, Nil};
    use std::fmt;
    use std::sync::Arc;

    #[derive(PartialEq, Clone)]
    pub enum List<T> {
        Cons(T, Arc<List<T>>),
        Nil,
    }

    impl<T> fmt::Display for List<T>
    where
        T: fmt::Display,
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Cons(ref head, ref tail) => {
                    write!(f, "{head} :: ", head = head);
                    write!(f, "{tail} ", tail = tail)
                }
                Nil => write!(f, "Nil"),
            }
        }
    }

    impl<T> List<T> {
        #[allow(dead_code)]
        pub fn flat_map<U>(&self, f: &Fn(T) -> List<U>) -> List<U>
        where
            T: Copy,
            T: Sized,
            T: Copy,
            T: PartialEq,
            U: Copy,
            U: Sized,
            U: PartialEq,
        {
            match *self {
                Cons(ref head, ref tail) => f(*head).concat(tail.flat_map(f)),
                Nil => Nil,
            }
        }
        #[allow(dead_code)]
        pub fn contains(&self, e: T) -> bool
        where
            T: PartialEq,
        {
            match *self {
                Cons(ref head, ref tail) => if *head == e {
                    true
                } else {
                    tail.contains(e)
                },
                Nil => false,
            }
        }
        #[allow(dead_code)]
        pub fn filter(&self, f: &Fn(T) -> bool) -> List<T>
        where
            T: Copy,
        {
            self.fold_right(Nil, &|x, y| {
                if f(x) {
                    Cons(x, Arc::new(y.filter(f)))
                } else {
                    y.filter(f)
                }
            })
        }
        #[allow(dead_code)]
        pub fn concat(&self, b: List<T>) -> List<T>
        where
            T: Copy,
            T: Sized,
        {
            self.fold_right(b, &|x, y| Cons(x, Arc::new(y)))
        }
        #[allow(dead_code)]
        pub fn fold_right<U>(&self, result: U, f: &Fn(T, U) -> U) -> U
        where
            T: Copy,
            T: Sized,
        {
            match *self {
                Cons(ref head, ref tail) => f(*head, tail.fold_right(result, f)),
                Nil => result,
            }
        }
        #[allow(dead_code)]
        pub fn fold_left<U>(&self, result: U, f: &Fn(U, T) -> U) -> U
        where
            T: Copy,
            T: Sized,
        {
            match *self {
                Cons(ref head, ref tail) => tail.fold_left(f(result, *head), f),
                Nil => result,
            }
        }
        #[allow(dead_code)]
        pub fn map<U>(&self, f: &Fn(T) -> U) -> List<U>
        where
            T: Copy,
            U: Copy,
        {
            self.fold_right(Nil, &|x, y| Cons(f(x), Arc::new(y)))
        }
    }

    #[allow(dead_code)]
    pub fn new_list<T>(args: Vec<T>) -> List<T>
    where
        T: Copy,
    {
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
    println!(
        "list.fold_left(0, &|x, y| x + y) = {result}",
        result = list.fold_left(0, &|x, y| x + y)
    );
    println!(
        "{list} contains 3 == {result}",
        list = list,
        result = list.contains(3)
    );
    println!(
        "{list} contains 6 == {result}",
        list = list,
        result = list.contains(6)
    );
}
