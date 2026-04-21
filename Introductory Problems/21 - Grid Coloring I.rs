pub fn solve() {
    cp::prepare!();
    sc!(n: usize, m: usize, mut c: [Chars; n]);
    debug_assert!(c.get(0).is_some() && c.get(0).unwrap().len() == m);

    for i in 0..n {
        for j in 0..m {
            if i + j & 1 == 0 {
                c[i][j] = (c[i][j] != 'A').then_some('A').unwrap_or('B');
            } else {
                c[i][j] = (c[i][j] != 'C').then_some('C').unwrap_or('D');
            }
        }
    }

    c.into_iter().for_each(|l| pp!(@ns @it l.into_iter()));
}

cp::main!();
