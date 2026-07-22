pub fn solve() {
    cp::prepare!();
    sc!(n: usize, g: [Bytes; n]);
    const M: usize = 1_000_000_007;
    let mut dp = vec![0usize; n + 1];
    dp[1] = 1;
    for row in g {
        for j in 1..=n {
            dp[j] = if row[j - 1] == b'.' {
                let s = dp[j] + dp[j - 1];
                s - M * ((s >= M) as usize)
            } else {
                0
            };
        }
    }
    pp!(dp[n]);
}
 
cp::main!();
