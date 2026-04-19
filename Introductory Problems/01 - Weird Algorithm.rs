pub fn solve() {
    cp::prepare!();
    sc!(a: usize);
    pp!(@it std::iter::successors(Some(a),
        |&x| (x != 1)
            .then_some(
                (x & 1 == 0)
                    .then_some(x >> 1)
                    .unwrap_or(3 * x + 1)
            )
    ));
}

cp::main!();
