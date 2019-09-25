use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::ptr;

type BeginAt = usize;
type EndAt = usize;

struct Hit(BeginAt, EndAt);

#[derive(Clone)]
struct Node<'a> {
    in_dict: bool,
    depth: usize,
    index: usize,
    parent: *const Node<'a>,
    suffix: Option<&'a Node<'a>>,
    dict_suffix: Option<&'a Node<'a>>,
    next: Vec<Option<Box<Node<'a>>>>,
}

struct Automaton<'a> {
    root: Box<Node<'a>>,
}

impl<'a> Node<'a> { 
    pub fn new(depth: usize, index: usize,
               parent: *const Node<'a>) -> Self {
        Node {
            in_dict: false,
            depth: depth,
            index: index,
            parent: parent,
            suffix: None,
            dict_suffix: None,
            next: vec![None; 256],
        }
    }
}

impl<'a> PartialEq for &Node<'a> {
    fn eq(&self, other: &Self) -> bool {
        (*self) as *const _ == (*other) as *const _
    }
}

impl<'a> Automaton<'a> {
    pub fn new(dict: &Vec<&str>) -> Self {
        let mut automaton = Automaton {
            root: Box::new(Node::new(0, 0, ptr::null())),
        };
        for word in dict.iter() {
            Self::insert(&mut automaton.root, word);
        }
        automaton.initialize();
        automaton
    }

    pub fn scan(&self, text: &str) -> Vec<Hit> {
        let mut hits = Vec::new();
        let mut current: &Node = self.root.borrow();
        for i in 0..text.len() {
            let mut temp = Some(current);
            while let Some(temp_) = temp {
                let index = text.chars().nth(i).unwrap() as usize;
                match temp_.next[index] {
                    Some(ref box_) => {  // Found match.
                        Self::retrieve(box_.borrow(), i + 1, &mut hits);
                        current = box_.borrow();
                        break;
                    },
                    None => {
                        temp = temp_.suffix;
                    },
                }
            }
        }
        hits
    }

    fn insert(node: &mut Node, word: &str) {
        if word.is_empty() {
            node.in_dict = true;
            return;
        }
        let index = word.chars().nth(0).unwrap() as usize;
        let raw_node: *const Node = node;
        let child: &mut Node =
            node.next[index]
                .get_or_insert(
                    Box::new(Node::new(node.depth + 1, index, raw_node)))
                .borrow_mut();
        Self::insert(child, &word[1..]);
    }

    fn retrieve(mut node: &Node, end_at: usize, hits: &mut Vec<Hit>) {
        if node.in_dict {
            hits.push(Hit(end_at - node.depth, end_at));
        }
        loop {
            match node.dict_suffix {
                Some(suf) => {
                    node = suf;
                },
                None => {
                    break;
                },
            }
            hits.push(Hit(end_at - node.depth, end_at));
        }
    }

    fn initialize(&mut self) {
        Self::traverse_mut(self.root.borrow_mut(), &|node| {
            let mut current: &Node;
            unsafe {
                current = node.parent.as_ref().unwrap();
            }
            loop {
                match current.suffix {
                    Some(suf) => {
                        if let Some(ref box_) = suf.next[node.index] {
                            node.suffix = Some(box_.borrow());
                            break;
                        }
                        current = suf;
                    },
                    None => {  // Reached root.
                        node.suffix = Some(current);
                        break;
                    },
                }
            }
        });
        Self::traverse_mut(self.root.borrow_mut(), &|node| {
            let mut current = node.suffix;
            loop {
                match current {
                    Some(suf) => {
                        if suf.in_dict {
                            node.dict_suffix = Some(suf);
                            break;
                        }
                        current = suf.suffix;
                    },
                    None =>  {  // Reached root.
                        break;
                    },
                }
            }
        });
    }

    fn traverse_mut<F>(node: &mut Node, transform: &F)
        where F: Fn(&mut Node) {
        for opt in node.next.iter_mut() {
            if let Some(child) = opt {
                transform(child);
                Self::traverse_mut(child, transform);
            }
        }
    }
}

fn main() {
    let dict = vec!["a", "ab", "bab", "bc", "bca", "c", "caa"];
    let text = "abccab";

    let matcher = Automaton::new(&dict);
    let hits = matcher.scan("abccab");
    println!("Found {} hit(s) in total in '{}'.", hits.len(), text);

    let mut results: HashMap<&str, Vec<BeginAt>> = HashMap::new();
    for hit in hits.iter() {
        let entry =
            results.entry(&text[hit.0..hit.1]).or_insert(Vec::new());
        entry.push(hit.0);
    }
    for (k, v) in results {
        print!("Occurrance of '{}': ", k);
        for begin_at in v.iter() {
            print!("@{} ", begin_at);
        }
        print!("\n");
    }
}
