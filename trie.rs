use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::boxed::Box;
use std::option::Option;

struct Node {
    in_dict: bool,
    children: Vec<Option<Box<Node>>>,
}

impl Node {
    pub fn new() -> Self {
        let mut node = Node {
            in_dict: false,
            children: Vec::new(),
        };

        for _ in 0..256 {
            node.children.push(None);
        }

        node
    }
}

struct Trie {
    root: Option<Box<Node>>,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: Some(Box::new(Node::new())),
        }
    }

    pub fn insert(&mut self, word: &str) -> () {
        Trie::insert_inl(&mut self.root, word);
    }

    pub fn find(&self, word: &str) -> bool {
        Trie::find_inl(&self.root, word)
    }

    fn insert_inl(node: &mut Option<Box<Node>>, remain: &str) -> () {
        let node_mut: &mut Node = node.as_mut().unwrap().borrow_mut();
        if remain.is_empty() {
            node_mut.in_dict = true;
        } else {
            let index = remain.chars().next().unwrap() as usize;
            let child = &mut node_mut.children[index];
            if child.is_none() {
                *child = Some(Box::new(Node::new()));
            }
            Trie::insert_inl(child, &remain[1..]);
        }
    }

    fn find_inl(node: &Option<Box<Node>>, remain: &str) -> bool {
        match node {
            &Some(ref node_) => {
                let inner: &Node = node_.borrow();
                if remain.is_empty() {
                    inner.in_dict
                } else {
                    let index = remain.chars().next().unwrap() as usize;
                    Trie::find_inl(&inner.children[index], &remain[1..])
                }
            }
            &None => false
        }
    }
}

fn main() {
    let mut trie = Trie::new();

    let dictionary = vec!["apple", "banana"];
    for word in dictionary {
        trie.insert(word);
    }

    let query = vec!["apple", "pineapple"];
    for word in query {
        if trie.find(word) {
            println!("{} exists in dictionary", word);
        } else {
            println!("{} is not found", word);
        }
    }
}
