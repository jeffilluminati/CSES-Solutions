pub fn solve() {
    cp::prepare!();
    sc!(n: u64);

    (1..=n).for_each(|i| {
        // no. of ways two knights can be placed on the board
        // - no. of ways two knights can attack each other.
        pp!((i * i * (i * i - 1) >> 1) - (4 * (i - 1) * (i - 2)));
    })
}

cp::main!();
