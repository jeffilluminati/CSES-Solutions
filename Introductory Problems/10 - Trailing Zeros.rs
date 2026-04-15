pub fn solve() {
    cp::prepare!();
    sc!(n: usize);

    pp!(std::iter::successors(Some(n / 5), |&x| { (x > 0).then(|| x / 5) }).sum::<usize>());
}

cp::main!();
