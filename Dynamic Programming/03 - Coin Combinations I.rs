pub fn solve() {
    cp::prepare!();
    const MOD: usize = 1_000_000_007;
    let ws = |a: usize, b: usize| (a + b) - MOD * (((a + b) >= MOD) as usize);
    sc!(n: usize, x: usize, c: [usize; n]);
 
    let mut dp = vec![0usize; x + 1];
    dp[0] = 1;
 
    for i in 1..=x {
        for &coin in c.iter() {
            if i >= coin {
                dp[i] = ws(dp[i], dp[i - coin]);
            }
        }
    }
 
    pp!(dp[x]);
}
 
cp::main!();
