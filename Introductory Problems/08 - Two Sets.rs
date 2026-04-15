pub fn solve() {
    cp::prepare!();
    sc!(n: usize);

    if (n * (n + 1) / 2) & 1 != 0 {
        pp!("NO");
        return;
    }

    let (mut a, mut b) = (Vec::new(), Vec::new());
    if n % 4 == 0 {
        for i in 1..=n / 4 {
            a.extend([i, n + 1 - i]);
            b.extend([n / 2 + i, n / 2 + 1 - i]);
        }
    } else {
        a.extend([1, 2]);
        b.push(3);

        for i in 0..(n - 3) / 4 {
            let x = 4 * i + 4;
            a.extend([x, x + 3]);
            b.extend([x + 1, x + 2]);
        }
    }

    pp!(
        "YES";
        a.len();
        @it a.iter();
        b.len();
        @it b.iter()
    );
}

cp::main!();
