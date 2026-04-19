pub fn solve() {
    cp::prepare!();
    sc!(n: usize, p: [usize; n]);

    pp!((0..1 << n).fold(usize::MAX, |ans, m| (0..n)
        .fold(0, |d, i| ((m & 1 << i) != 0)
            .then_some(d + p[i])
            .unwrap_or(d - p[i]))
        .min(ans)));
}

cp::main!();
