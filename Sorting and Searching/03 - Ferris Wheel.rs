pub fn solve() {
    cp::prepare!();
    sc!(n: usize, x: usize, mut p: [usize; n]);

    p.sort_unstable();
    let (mut l, mut r) = (0, n - 1);
    let mut ans = 0;
    while l < r {
        if p[l] + p[r] <= x {
            l += 1;
        }

        r -= 1;
        ans += 1;
    }

    //odd case
    if l == r { ans += 1; }
    pp!(ans);
}

cp::main!();
