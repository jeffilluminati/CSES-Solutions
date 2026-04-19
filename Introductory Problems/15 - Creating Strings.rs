pub fn solve() {
    cp::prepare!();
    sc!(s: String);

    let mut ans = Vec::<String>::new();
    let mut cur = String::with_capacity(s.len());

    let mut freq = [0u8; 26];
    s.bytes().for_each(|b| {
        freq[(b - b'a') as usize] += 1;
    });

    fn create(cur: &mut String, ans: &mut Vec<String>, freq: &mut [u8; 26], n: usize) {
        if cur.len() == n {
            ans.push(cur.clone());
            return;
        }

        for i in 0..26 {
            if freq[i] > 0 {
                freq[i] -= 1;
                cur.push((i as u8 + b'a') as char);
                create(cur, ans, freq, n);
                cur.pop();
                freq[i] += 1;
            }
        }
    }

    create(&mut cur, &mut ans, &mut freq, s.len());

    pp!(ans.len());
    pp!(@lf @it ans.into_iter());
}

cp::main!(large_stack);
