const MOD: usize = 1000000007;

pub fn solve() {
    cp::prepare!();

    sc!(n: usize);

    let mut dp = vec![0usize; n + 3];
    dp[0] = 1;

    for i in 1..=n {
        for j in 1..=6.min(i) {
            dp[i] = (dp[i] + dp[i-j]) % MOD;
        }
    }

    pp!(dp[n]);
}

cp::main!();
