pub fn solve() {
    cp::prepare!();
    sc!(s: Bytes);

    let n = s.len();
    let mut freq = [0usize; 26];
    s.into_iter().for_each(|&c| freq[(c - b'A') as usize] += 1);

    let (o1, o2) = {
        let mut odd = freq
            .iter()
            .enumerate()
            .filter_map(|(i, &cnt)| (cnt % 2 == 1).then_some(i as u8 + b'A'));
        (odd.next(), odd.next())
    };

    if o2.is_some() {
        pp!("NO SOLUTION");
        return;
    }

    let mut ans = Vec::<u8>::with_capacity(n);
    ans.extend(
        (0..26)
            .flat_map(|i| std::iter::repeat(i + b'A').take(freq[i as usize] / 2))
            .chain(o1.into_iter())
            .chain(
                (0..26)
                    .rev()
                    .flat_map(|i| std::iter::repeat(i + b'A').take(freq[i as usize] / 2)),
            ),
    );

    pp!(@b ans);
}

cp::main!();
