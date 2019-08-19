impl Solution {
    pub fn last_substring(s: String) -> String {
        let mut i = 0;
        let mut j = 1;
        while let Some(atj) = s.chars().nth(j) {
            let ati = s.chars().nth(i).unwrap();
            if atj > ati {
                i = j;
            } else if atj == ati {
                let mut k = 1;
                while let Some(atjk) = s.chars().nth(j + k) {
                    let atik = s.chars().nth(i + k).unwrap();
                    if (atjk > atik) {
                        i = j;
                        break;
                    } else if (atjk < atik) {
                        break;
                    }
                    k += 1;
                }
                j += k - 1;
            }
            j += 1;
        }
        String::from(&s[i..])
    }
}