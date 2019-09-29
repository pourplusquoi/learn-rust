// Bit manipulation.
impl Solution {
    pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut res: Vec<bool> = Vec::new();
        let mut map: Vec<u32> = vec![0; s.len() + 1];
        let mut tmp = 0;
        for (i, ch) in (1..).zip(s.chars()) {
            tmp ^= 1 << (ch as u32 - 'a' as u32);
            map[i] = tmp;
        }
        for q in queries.iter() {
            let lo = q[0] as usize;
            let hi = q[1] as usize;
            res.push((map[lo] ^ map[hi + 1]).count_ones() / 2 <= q[2] as u32);
        }
        res
    }
}