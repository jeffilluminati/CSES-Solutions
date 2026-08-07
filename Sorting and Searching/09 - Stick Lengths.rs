pub fn solve() {
    cp::prepare!();
    sc!(n: usize, mut p: [usize; n]);

    p.sort_unstable();
    let med = p[n / 2];
    pp!(p.iter().map(|&x| x.abs_diff(med)).sum::<usize>());
}

cp::main!();
