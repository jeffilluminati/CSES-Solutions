const INF: u32 = u32::MAX - 1;

pub fn solve() {
    cp::prepare!();
    sc!(n: usize, x: usize);
    sc!(mut coins: [usize; n]);
    
    coins.sort_unstable();
    coins.dedup();

    let mut dp = vec![INF; x + 1];
    dp[0] = 0;

    for &c in &coins {
        for j in c..x+1 {
            dp[j] = dp[j].min(dp[j - c] + 1);
        }
    }

    if dp[x] == INF {
        pp!(-1)
    } else {
        pp!(dp[x])
    }
}

cp::main!();
