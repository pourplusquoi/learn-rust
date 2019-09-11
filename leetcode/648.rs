struct Node {
    is_word: bool,
    next: Option<Vec<Box<Node>>>,
}

struct Trie {
    root: Node,
}

impl Node {
    pub fn new() -> Self {
        Node {
            is_word: false,
            next: None,
        }
    }
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: Node::new(),
        }
    }
    
    pub fn insert(&mut self, word: &str) {
        Trie::insert_impl(&mut self.root, word);
    }
    
    pub fn find(&self, word: &str) -> usize {
        Trie::find_impl(&self.root, word, 0)
    }
    
    fn insert_impl(node: &mut Node, word: &str) {
        match word.chars().nth(0) {
            None => {
                node.is_word = true;
            },
            Some(c) => {
                if node.next.is_none() {
                    let mut val = Vec::new();
                    for _ in 0..26 {
                        val.push(Box::new(Node::new()));
                    }
                    node.next = Some(val);
                }
                if let Some(next_) = node.next.as_mut() {
                    let idx = c as usize - 'a' as usize;
                    Trie::insert_impl(&mut next_[idx], &word[1..]);
                }
            },
        }
    }
    
    fn find_impl(node: &Node, word: &str, count: usize) -> usize {
        if node.is_word || word.is_empty() {
            return count;
        }
        match node.next.as_ref() {
            None => count + word.len(),
            Some(next_) => {
                let idx = word.chars().nth(0).unwrap() as usize - 'a' as usize;
                Trie::find_impl(&next_[idx], &word[1..], count + 1)
            }
        }
    }
}

impl Solution {    
    pub fn replace_words(dict: Vec<String>, sentence: String) -> String {
        let mut trie = Trie::new();
        for word in dict.iter() {
            trie.insert(word);
        }
        let mut res = String::from("");
        let words: Vec<&str> = sentence.split(' ').collect();
        for word in words {
            let word_ = &word[0..trie.find(word)];
            res = if res.is_empty() {
                format!("{}", word_)
            } else {
                format!("{} {}", res, word_)
            }
        }
        res
    }
}