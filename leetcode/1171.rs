use std::collections::HashMap;

impl Solution {
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut idx = 0;
        let mut sum = 0;
        let mut seen: HashMap<i32, usize> = HashMap::new();
        seen.insert(sum, 0);

        let mut intv: Vec<(usize, usize)> = Vec::new();
        let mut temp = &head;
        while let Some(ref node) = temp {
            idx += 1;
            sum += node.val;
            if seen.contains_key(&sum) {
                intv.push((*seen.get(&sum).unwrap(), idx));
            }
            seen.insert(sum, idx);
            temp = &node.next;
        }

        intv.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0));
        let mut bit_mask: Vec<bool> = vec![true; idx];
        let mut i = 0;
        while i < intv.len() {
            for k in intv[i].0..intv[i].1 {
                bit_mask[k] = false;
            }
            i += intv.iter()
                     .skip(i)
                     .take_while(|x| x.0 < intv[i].1)
                     .count();
        }

        let mut prehead = Box::new(ListNode::new(0));
        let mut pointer = &mut prehead;
        let mut current = head;
        for j in 0..idx {
            if bit_mask[j] {
                pointer.next = Some(
                    Box::new(ListNode::new(current.as_ref().unwrap().val)));
                pointer = pointer.next.as_mut().unwrap();
            }
            current = current.unwrap().next;
        }
        prehead.next
    }
}