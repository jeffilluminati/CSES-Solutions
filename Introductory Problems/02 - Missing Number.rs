pub fn solve() {
    cp::prepare!();
    sc!(n: usize, a: [usize; n-1]);

    pp!((1..=n).fold(0usize, |acc, i| acc ^ i) ^ a.iter().fold(0usize, |acc, i| acc ^ i));

    // This is O(nlogn)
    // a.sort_unstable();
    // pp!((1..=n).find(|&x| x == n || a[x - 1] != x).unwrap());
}

cp::main!();
