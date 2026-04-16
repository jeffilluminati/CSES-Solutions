pub fn solve() {
    cp::prepare!();
    sc!(t: u32);

    for _ in 0..t {
        sc!(a: usize, b: usize);
        (a.max(b) > 2 * a.min(b) || (a + b) % 3 != 0)
            .then(|| pp!("NO"))
            .unwrap_or_else(|| pp!("YES"));
    }
}

cp::main!();
