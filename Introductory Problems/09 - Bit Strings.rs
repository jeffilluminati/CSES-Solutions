pub fn solve() {
    cp::prepare!();

    sc!(n: usize);

    const MOD: usize = 1_000_000_007;
    pp!(
        std::iter::successors(Some((n, 2usize, 1usize)), |&(x, y, z)| {
            (x > 0).then(|| {
                (
                    x >> 1,
                    (y * y) % MOD,
                    (x & 1 == 1).then_some((z * y) % MOD).unwrap_or(z),
                )
            })
        })
        .last()
        .unwrap()
        .2
    );

    // Breaks because (1 << n) blows up
    // pp!((1 << n) % MOD);
}

cp::main!();
