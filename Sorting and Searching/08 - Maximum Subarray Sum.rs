pub fn solve() {
    cp::prepare!();
    sc!(n: usize, x: [i64; n]);

    let (mut res, mut curr_max) = (x[0], x[0]); 
    for i in 1..n {
        curr_max = x[i].max(curr_max+x[i]);
        res = res.max(curr_max);
    }

    pp!(res);
}

cp::main!();
