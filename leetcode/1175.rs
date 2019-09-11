fn binary_search(primes: [i32; 26], tar: i32) -> usize {
    let mut lo = 0;
    let mut hi = primes.len() - 1;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if primes[mid] <= tar {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

fn factor(num: usize, modulo: usize) -> usize {
    let mut res = 1;
    for i in 2..=num {
        res = (res * i) % modulo;
    }
    res
}

impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        let modulo = 1_000_000_007;
        let primes = [ 2,  3,  5,  7, 11, 13, 17, 19, 23, 29,
                      31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
                      73, 79, 83, 89, 97, 101];
        let prime_count = binary_search(primes, n);
        let res = (factor(prime_count, modulo) *
            factor(n as usize - prime_count, modulo)) % modulo;
        res as i32
    }
}