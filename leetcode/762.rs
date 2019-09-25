impl Solution {
    pub fn count_prime_set_bits(l: i32, r: i32) -> i32 {
        let primes = [
            0, 0, 1, 1, 0, 1, 0, 1, 0, 0,
            0, 1, 0, 1, 0, 0, 0, 1, 0, 1];
        (l..=r).filter(|x|
                    primes[x.count_ones() as usize] > 0)
               .count() as i32
    }
}