pub fn solve() {
    cp::prepare!();
    sc!(n: usize, m: usize, k: usize);
    sc!(mut a: [usize; n]);
    sc!(mut b: [usize; m]);
    a.sort_unstable();
    b.sort_unstable();

    let (mut i, mut j) = (0, 0);
    let mut ans = 0;

    while i < n && j < m {
        if a[i] + k < b[j] {
            i += 1;
        } else if b[j] + k < a[i] {
            j += 1;
        } else {
            ans += 1;
            i += 1;
            j += 1;
        }
    }

    pp!(ans);
}

cp::main!();
