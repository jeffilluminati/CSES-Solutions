pub fn solve() {
    cp::prepare!();
    sc!(s: String);

    let mut freq = [0usize; 26];
    s.bytes().for_each(|b| freq[(b - b'A') as usize] += 1);

    let odd = freq
        .iter()
        .enumerate()
        .filter_map(|(i, &cnt)| (cnt % 2 == 1).then_some(i as u8 + b'A'))
        .collect::<Vec<_>>();

    if odd.len() > 1 {
        pp!("NO SOLUTION");
        return;
    }

    pp!(String::from_utf8(
        (0..26)
            .flat_map(|i| std::iter::repeat(i as u8 + b'A').take(freq[i] / 2))
            .chain(odd.into_iter())
            .chain(
                (0..26)
                    .rev()
                    .flat_map(|i| std::iter::repeat(i as u8 + b'A').take(freq[i] / 2)),
            )
            .collect::<Vec<_>>()
    )
    .unwrap());
}

cp::main!();
