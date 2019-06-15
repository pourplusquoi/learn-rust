use std::boxed::Box;

#[derive(Clone)]
pub enum Stack<T> {
    Empty,
    Cons(T, Box<Stack<T>>),
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack::Empty
    }

    pub fn is_empty(&self) -> bool {
        match self {
            &Stack::Empty => true,
            _ => false,
        }
    }

    pub fn push(self, elem: T) -> Self {
        Stack::Cons(elem, Box::new(self))
    }

    pub fn pop(self) -> Option<(Self, T)> {
        match self {
            Stack::Empty => None,
            Stack::Cons(elem, tail) => Some((*tail, elem)),
        }
    }

    pub fn top(&self) -> Option<&T> {
        match self {
            &Stack::Empty => None,
            &Stack::Cons(ref elem, _) => Some(elem),
        }
    }

    pub fn next(&self) -> &Self {
        match self {
            &Stack::Empty => self,
            &Stack::Cons(_, ref tail) => tail,
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut current = self;
        while let &Stack::Cons(ref elem, ref tail) = current {
            current = tail;
            write!(f, "{}", elem)?;
            match **tail {
                Stack::Empty => (),
                _ => {
                    write!(f, " -> ")?;
                }
            }
        }
        Ok(())
    }
}

fn main() {
    let mut stack = Stack::new();
    println!("Stack: {}", stack);

    for i in (0..10).rev() {
       stack = stack.push(i);
    }
    println!("Top: {}", stack.top().unwrap());
    println!("Next: {}", stack.next());

    for _ in 0..5 {
        stack = stack.pop().unwrap().0;
    }
    println!("Stack: {}", stack);
}