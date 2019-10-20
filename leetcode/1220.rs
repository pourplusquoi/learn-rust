impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let modulo = 1_000_000_007;
        let mut dp: [u32; 5] = [1; 5];
        for _ in 1..n {
            let a = (dp[1] + dp[2] + dp[4]) % modulo;
            let e = (dp[0] + dp[2]) % modulo;
            let i = (dp[1] + dp[3]) % modulo;
            let o = dp[2];
            let u = (dp[2] + dp[3]) % modulo;
            dp = [a, e, i, o, u];
        }
        dp.iter().fold(0, |acc, x| (acc + x) % modulo) as i32
    }
}