use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct DinnerPlates {
    cap: usize,
    plates: Vec<Vec<i32>>,
    max_heap: BinaryHeap<usize>,  // can pop
    min_heap: BinaryHeap<Reverse<usize>>,  // can push
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DinnerPlates {

    fn new(capacity: i32) -> Self {
        DinnerPlates {
            cap: capacity as usize,
            plates: Vec::new(),
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
        }
    }
    
    fn push(&mut self, val: i32) {
        if let Some(&Reverse(idx)) = self.min_heap.peek() {
            if self.plates[idx].is_empty() {
                self.max_heap.push(idx);
            }
            self.plates[idx].push(val);
            if self.plates[idx].len() == self.cap {
                self.min_heap.pop();  // can no longer push
            }
        } else {
            self.max_heap.push(self.plates.len());
            if self.cap > 1 {
                self.min_heap.push(Reverse(self.plates.len()));
            }
            self.plates.push(vec![val]);
        }
    }
    
    fn pop(&mut self) -> i32 {
        while self.max_heap.peek().is_some() && 
              self.plates[*self.max_heap.peek().unwrap()].is_empty() {
            self.max_heap.pop();
        }
        if let Some(&idx) = self.max_heap.peek() {
            if self.plates[idx].len() == self.cap {
                self.min_heap.push(Reverse(idx));
            }
            let val = self.plates[idx].pop().unwrap();
            if self.plates[idx].is_empty() {
                self.max_heap.pop();  // can no longer pop
            }
            val
        } else {
            -1
        }
    }
    
    fn pop_at_stack(&mut self, index: i32) -> i32 {
        let idx = index as usize;
        if let Some(stack) = self.plates.iter_mut().nth(idx).as_mut() {
            if stack.len() == self.cap {
                self.min_heap.push(Reverse(idx));
            }
            if let Some(val) = stack.pop() {
                val
            } else {
                -1
            }
        } else {
            -1
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