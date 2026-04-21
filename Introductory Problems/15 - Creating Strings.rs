pub fn solve() {
    cp::prepare!();
    sc!(s: Bytes);

    let n = s.len() as u8;
    let mut freq = [0u8; 26];
    s.into_iter().for_each(|b| freq[(b - b'a') as usize] += 1);
    let unique = freq.iter().filter(|&&f| f != 0).count();

    let n = n as usize;
    let mut cur = Vec::<u8>::with_capacity(n);
    let mut next = Vec::<u8>::with_capacity(n + 1);
    // The final vector is of max size (n+1)! / (n-unique)!
    let ans_cap = ((n - unique + 1)..=(n + 1)).fold(1, |acc, x| acc * x);
    let mut ans = Vec::<u8>::with_capacity(ans_cap);
    let mut cnt = 0;

    next.push(0);

    while let Some(i) = next.last_mut() {
        if cur.len() == n {
            ans.extend(cur.iter());
            ans.push(b'\n');
            cnt += 1;

            next.pop();
            if let Some(j) = cur.pop() {
                freq[(j - b'a') as usize] += 1;
            }
            continue;
        }

        while *i < 26 && freq[*i as usize] == 0 {
            *i += 1;
        }

        if *i == 26 {
            next.pop();
            if let Some(j) = cur.pop() {
                freq[(j - b'a') as usize] += 1;
            }
        } else {
            let j = *i;
            *i += 1;
            freq[j as usize] -= 1;
            cur.push(j + b'a');
            next.push(0);
        }
    }

    pp!(cnt);
    pp!(@b ans);
    // pp!(ans_cap.abs_diff(ans.len()), ans_cap >= ans.len());
}

cp::main!();
