use std::collections::HashMap;

struct Transaction<'a> {
    name: &'a str,
    time: u16,
    amount: u16,
    city: &'a str,
    valid: bool,
}

impl<'a> Transaction<'a> {
    pub fn from(transaction: &str) -> Transaction {
        let words: Vec<&str> = transaction.split(',').collect();
        Transaction {
            name: words[0],
            time: words[1].parse::<u16>().unwrap(),
            amount: words[2].parse::<u16>().unwrap(),
            city: words[3],
            valid: true,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{},{},{},{}", self.name, self.time, self.amount, self.city)
    }
}

fn index_mut<'a, T>(vec: &'a mut Vec<&mut T>, idx: usize) -> &'a mut T {
    vec.iter_mut().nth(idx).unwrap()
}

impl Solution {
    fn conditional_push(mut res: Vec<String>, tran: &mut Transaction) -> Vec<String> {
        if tran.valid {
            tran.valid = false;
            res.push(tran.to_string());
        }
        res
    }

    pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
        let mut trans: Vec<Transaction> = Vec::new();
        for t in transactions.iter() {
            trans.push(Transaction::from(t));
        }

        let mut map: HashMap<&str, Vec<&mut Transaction>> = HashMap::new();
        for t in trans.iter_mut() {
            let val = map.entry(t.name).or_insert(Vec::new());
            val.push(t);
        }

        let mut res: Vec<String> = Vec::new();
        for (_, val) in map.iter_mut() {
            val.sort_by(|lhs, rhs| lhs.time.cmp(&rhs.time));
            // Check for name and city.
            let mut i = 0;
            let mut j = 1;
            while {
                while j < val.len() && val[j].city == val[i].city {
                    j += 1;
                }
                j < val.len()
            } {
                while i < j && val[j].time - val[i].time > 60 {
                    i += 1;
                }
                while i < j {
                    res = Self::conditional_push(res, index_mut(val, i));
                    i += 1;
                }
                let mut k = j;
                while k < val.len() && val[k].city == val[j].city &&
                      val[k].time - val[j - 1].time <= 60 {
                    k += 1;
                }
                while j < k {
                    res = Self::conditional_push(res, index_mut(val, j));
                    j += 1;
                }
            }
            // Check for amount.
            for t in val.iter_mut() {
                if t.amount > 1000 {
                    res = Self::conditional_push(res, t);
                }
            }
        }
        res
    }
}