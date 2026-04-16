pub fn solve() {
    cp::prepare!();
    sc!(n: u8);

    (0..(1 << n) as usize).fold([false; 17], |mut acc, i| {
        acc[(i as u16).trailing_zeros() as usize] ^= true;
        pp!(@ns @it (0..n).rev().map(|i| acc[i as usize].then_some(1).unwrap_or(0)));
        acc
    });
}

cp::main!();
