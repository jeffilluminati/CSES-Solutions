use std::collections::BTreeMap;

pub fn solve() {
    cp::prepare!();
    sc!(n: usize, m: usize);
    sc!(mut h: [isize; n], mut t: [isize; m]);
    let mut tickets = BTreeMap::<isize, usize>::new();
    h.iter().for_each(|&x| *tickets.entry(x).or_default() += 1);

    let mut res = Vec::<isize>::with_capacity(m);

    for budget in t {
        if let Some((&price, _)) = tickets.range(..=budget).next_back() {
            res.push(price);

            let cnt = tickets.get_mut(&price).unwrap();
            *cnt -= 1;
            if *cnt == 0 { tickets.remove(&price); }
        } else {
            res.push(-1);
        }
    }

    pp!(@lf @it res);
}

cp::main!();
