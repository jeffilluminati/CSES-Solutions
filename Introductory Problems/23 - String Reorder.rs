pub fn solve() {
    cp::prepare!();
    sc!(s: String);
    let n = s.len();

    let mut freq = [0u32; 26];
    s.bytes().for_each(|b| freq[(b - b'A') as usize] += 1);

    let mut ans = String::with_capacity(n);

    let mut prev = 27;
    for _ in 0..n {
        let mut solved = false;
        let mut ch = 0;
        while !solved && ch < 26 {
            if freq[ch] > 0 && prev != ch {
                freq[ch] -= 1;
                let mut ch2 = 0;
                let mut total = 0;
                for ch3 in 0..26 {
                    (freq[ch2] < freq[ch3]).then(|| ch2 = ch3);
                    total += freq[ch3];
                }
                if freq[ch2] <= (total + 1) / 2 && freq[ch] <= total / 2 {
                    ans.push((ch as u8 + b'A') as char);
                    prev = ch;
                    solved = true;
                } else {
                    freq[ch] += 1;
                }
            }

            ch += 1;
        }

        if !solved {
            pp!(-1);
            return;
        }
    }

    pp!(ans);
}

cp::main!();
