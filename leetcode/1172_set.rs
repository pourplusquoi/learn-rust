use std::collections::BTreeSet;

struct DinnerPlates {
    capacity: usize,
    plates: Vec<Vec<i32>>,
    non_empty: BTreeSet<usize>,
    non_full: BTreeSet<usize>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DinnerPlates {

    fn new(capacity: i32) -> Self {
        DinnerPlates {
            capacity: capacity as usize,
            plates: Vec::new(),
            non_empty: BTreeSet::new(),
            non_full: BTreeSet::new(),
        }
    }
    
    fn push(&mut self, val: i32) {
        match self.non_full.iter().nth(0) {
            None => {  // Every stack is full.
                if self.capacity > 1 {
                    self.non_full.insert(self.plates.len());
                }
                self.non_empty.insert(self.plates.len());
                self.plates.push(vec![val]);
            },
            Some(&idx) => {
                self.plates[idx].push(val);
                if self.plates[idx].len() == self.capacity {
                    self.non_full.remove(&idx);
                }
                self.non_empty.insert(idx);
            },
        }
    }
    
    fn pop(&mut self) -> i32 {
        match self.non_empty.iter().last() {
            None => {  // Every stack is empty.
                -1
            },
            Some(&idx) => {
                let val = self.plates[idx].pop().unwrap();
                if self.plates[idx].is_empty() {
                    self.non_empty.remove(&idx);
                }
                self.non_full.insert(idx);
                val
            },
        }
    }
    
    fn pop_at_stack(&mut self, index: i32) -> i32 {
        if index < 0 || index >= self.plates.len() as i32 {
            return -1;
        }
        let idx = index as usize;
        match self.plates[idx].pop() {
            None => -1,
            Some(val) => {
                if self.plates[idx].is_empty() {
                    self.non_empty.remove(&idx);
                }
                self.non_full.insert(idx);
                val
            },
        }
    }
}

/**
 * Your DinnerPlates object will be instantiated and called as such:
 * let obj = DinnerPlates::new(capacity);
 * obj.push(val);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.pop_at_stack(index);
 */