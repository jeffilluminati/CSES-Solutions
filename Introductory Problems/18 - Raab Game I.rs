pub fn solve() {
    cp::prepare!();
    sc!(t: usize);

    for _ in 0..t {
        sc!(n: i16, a: i16, b: i16);

        let ties = n - a - b;
        let n = n - ties;
        if ties < 0 || n < 0 || a + b != n || ((a == 0 || b == 0) && a + b != 0) {
            pp!("NO");
            continue;
        }

        pp!("YES");
        pp!(@it (1..=(n+ties)));
        pp!(@it (1..=n).map(|x| {
            if x+a <= n {
                x+a
            } else {
                x+a-n
            }
        }).chain(n+1..=(n+ties)));
    }
}

cp::main!();
