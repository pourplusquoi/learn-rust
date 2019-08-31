use std::collections::HashMap;
use std::collections::HashSet;

struct Transaction<'a> {
    name: &'a str,
    time: u16,
    amount: u16,
    city: &'a str,
}

impl<'a> Transaction<'a> {
    pub fn from(transaction: &str) -> Transaction {
        let words: Vec<&str> = transaction.split(',').collect();
        Transaction {
            name: words[0],
            time: words[1].parse::<u16>().unwrap(),
            amount: words[2].parse::<u16>().unwrap(),
            city: words[3],
        }
    }

    pub fn to_string(&self) -> String {
        format!("{},{},{},{}", self.name, self.time, self.amount, self.city)
    }
}

impl Solution {
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

        let mut hs: HashSet<String> = HashSet::new();
        for (_, val) in map.iter_mut() {
            val.sort_by(|lhs, rhs| lhs.time.cmp(&rhs.time));
            let mut prev = 0;
            let mut curr = val.iter()
                              .take_while(|x| x.city == val[0].city)
                              .count();
            while curr < val.len() {
                hs = val.iter()
                         .skip(prev)
                         .skip_while(|ts| val[curr].time - ts.time > 60)
                         .take_while(|ts| ts.city != val[curr].city)
                         .fold(hs, |mut set, ts| {
                             set.insert(ts.to_string());
                             return set;
                          });
                hs = val.iter()
                        .skip(curr)
                        .take_while(|ts|
                            ts.time - val[curr - 1].time <= 60 &&
                            ts.city == val[curr].city)
                        .fold(hs, |mut set, ts| {
                            set.insert(ts.to_string());
                            return set;
                         });
                prev = curr;
                curr += val.iter()
                           .skip(prev)
                           .take_while(|ts| ts.city == val[prev].city)
                           .count();
            }

            for ts in val.iter_mut().filter(|x| x.amount > 1000) {
                hs.insert(ts.to_string());
            }
        }
        hs.iter().cloned().collect()
    }
}