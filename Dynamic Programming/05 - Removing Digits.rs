pub fn solve() {
    cp::prepare!();
    use cp::tools::ToDigitSequence;
    sc!(n: usize);
    let mut dp = vec![0usize; n + 1];
    dp[0] = 0;
 
    for i in 1..=n {
        dp[i] = 1 + i
            .to_digit_sequence()
            .into_iter()
            .filter(|&x| x != 0)
            .map(|x| dp[i - x])
            .min()
            .unwrap();
    }
 
    pp!(dp[n]);
}
 
cp::main!();
