pub fn solve() {
    cp::prepare!();
    sc!(t: u32);

    for _ in 0..t {
        sc!(y: usize, x: usize);

        let l: usize = x.max(y) - 1;
        pp!(match (l % 2 == 0, x < y) {
            (false, false) => l * l + y,
            (false, true) => (l + 2) * l - x + 2,
            (true, false) => (l + 2) * l - y + 2,
            (true, true) => l * l + x,
        });
    }
}

cp::main!();
