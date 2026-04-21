pub fn solve() {
    cp::prepare!();

    sc!(n: u8);
    let len = (1 << n as usize) - 1;
    pp!(len);

    let mut v = Vec::with_capacity(len);
    v.push((n, 1, 3));

    while let Some((count, src, dst)) = v.pop() {
        if count == 1 {
            pp!(src, dst);
        } else {
            let md = 6 - src - dst;
            v.push((count - 1, md, dst));
            v.push((1, src, dst));
            v.push((count - 1, src, md));
        }
    }
}

cp::main!();
