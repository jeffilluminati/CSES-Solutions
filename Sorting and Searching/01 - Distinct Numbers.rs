pub fn solve() {
    cp::prepare!();
    sc!(n: usize, mut x: [usize; n]);

    x.sort_unstable();

    pp!(x.windows(2).into_iter().fold(1usize, |acc, x| acc + ((x[0] != x[1]) as usize)));
}

cp::main!();
