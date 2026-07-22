pub fn solve() {
    cp::prepare!();
    sc!(n: usize, x: u32, mut a: [u32; n]);

    let mut a = a.iter().enumerate().collect::<Vec<_>>();
    a.sort_unstable_by_key(|x| x.1);

    let (mut l, mut r) = (0, n-1);
    while l < r {
        if a[l].1 + a[r].1 == x {
            pp!(a[l].0+1, a[r].0+1);
            return ;
        } else if a[l].1 + a[r].1 > x {
            r -= 1;
        } else {
            l += 1;
        }
    }

    pp!("IMPOSSIBLE");

}

cp::main!();
