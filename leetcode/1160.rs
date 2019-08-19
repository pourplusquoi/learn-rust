impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut count: Vec<u32> = vec![0; 26];
        for c in chars.chars() {
            count[c as usize - 'a' as usize] += 1;
        }
        let mut res = 0;
        for word in &words {
            let mut word_count: Vec<u32> = vec![0; 26];
            for c in word.chars() {
                word_count[c as usize - 'a' as usize] += 1;
            }
            let mut is_good = true;
            for i in 0..26 {
                if word_count[i] > count[i] {
                    is_good = false;
                    break;
                }
            }
            if is_good {
                res += word.len() as i32;
            }
        }
        res
    }
}