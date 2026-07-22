pub fn solve() {
    cp::prepare!();
    const MOD: usize = 1_000_000_007;
 
    sc!(n: usize, x: usize, c: [usize; n]);
    let mut dp = vec![0usize; x + 1];
    dp[0] = 1;
 
    for i in 0..n {
        for w in 0..=x {
            if w >= c[i] {
                dp[w] += dp[w - c[i]];
                dp[w] = (dp[w] >= MOD).then_some(dp[w] - MOD).unwrap_or(dp[w]);
            }
        }
    }
 
    pp!(dp[x]);
}
 
cp::main!();
