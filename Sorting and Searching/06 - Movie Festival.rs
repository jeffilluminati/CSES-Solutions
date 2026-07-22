pub fn solve() {
    cp::prepare!();
    sc!(n: u32, mut movies: [(u32, u32); n as usize]);

    movies.sort_unstable_by_key(|&(_, end)| end);

    let (mut res, mut prev) = (0, 0);

    for (start, end) in movies {
        if start >= prev {
            res += 1;
            prev = end;
        }
    }

    pp!(res);
}

cp::main!();
