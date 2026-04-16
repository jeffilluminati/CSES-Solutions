pub fn solve() {
    cp::prepare!();
    sc!(n: u8);

    let mut acc = [false; 17];
    (0..(1 << n) as usize).for_each(|i| {
        acc[(i as u16).trailing_zeros() as usize] ^= true;
        pp!(@ns @it (0..n).rev().map(|i| acc[i as usize] as u8));
    });
}

cp::main!();
