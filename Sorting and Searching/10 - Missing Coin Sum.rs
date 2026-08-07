pub fn solve() {
    cp::prepare!();

    sc!(n: usize, mut x: [usize; n]);
    x.sort_unstable();

    let mut val = 1;
    for c in x {
        if c > val {
            break;
        }
        val += c;
    }

    pp!(val);
}

cp::main!();
