pub fn solve() {
    cp::prepare!();
    sc!(n: usize, X: [u64; n]);

    pp!(X
        .iter()
        .fold((0u64, 0u64), |(moves, prev), &x| {
            (x < prev)
                .then_some((moves + prev - x, prev))
                .unwrap_or((moves, x))
        })
        .0);
}

cp::main!();
