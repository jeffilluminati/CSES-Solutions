pub fn solve() {
    cp::prepare!();
    sc!(q: u16);

    for _ in 0..q {
        sc!(mut k: u64);

        let (mut len, mut start) = (1u64, 1u64);
        while 9 * start * len < k {
            k -= 9 * start * len;
            len += 1;
            start *= 10;
        }

        k -= 1;
        pp!((start + k / len) / (0..(len - 1 - (k % len))).fold(1u64, |acc, _| acc * 10) % 10);
    }
}

cp::main!();
