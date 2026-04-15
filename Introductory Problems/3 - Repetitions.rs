pub fn solve() {
    cp::prepare!();
    sc!(s: String);

    pp!(s
        .bytes()
        .fold((1u32, 1u32, 0u8), |(best, curr, prev), &c| {
            (c == prev)
                .then_some((best.max(curr + 1), curr + 1, prev))
                .unwrap_or((best, 1, c))
        })
        .0);
}

cp::main!();
