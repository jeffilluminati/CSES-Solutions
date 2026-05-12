pub fn solve() {
    cp::prepare!();
    sc!(n: usize, m: usize, mut c: [Bytes; n]);
    debug_assert!(c.get(0).is_some() && c.get(0).unwrap().len() == m);

    for i in 0..n {
        for j in 0..m {
            c[i][j] = match (c[i][j] <= b'B', (i + j) & 1 != 0) {
                (false, false) => b'A',
                (false, true) => b'B',
                (true, false) => b'C',
                (true, true) => b'D',
            }
        }
    }

    pp!(@lf @itb c);
}

cp::main!();
