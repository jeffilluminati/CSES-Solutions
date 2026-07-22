pub fn solve() {
    cp::prepare!();
    sc!(n: usize);
    let mut events = Vec::<(usize, i32)>::with_capacity(2 * n);

    for _ in 0..n {
        sc!(a: usize, b: usize);
        events.push((a, 1));
        events.push((b, -1));
    }

    events.sort_unstable_by(|&(a1, a2), &(b1, b2)| a1.cmp(&b1).then(a2.cmp(&b2)));

    let (mut cur, mut ans) = (0, 0);

    for (_, delta) in events {
        cur += delta;
        ans = ans.max(cur);
    }

    pp!(ans);
}

cp::main!();
