pub fn solve() {
    cp::prepare!();
    sc!(n: usize);

    if n == 2 || n == 3 {
        pp!("NO SOLUTION");
        return;
    }

    let mut my_vec = (1..=n).collect::<Vec<usize>>();
    my_vec.sort_unstable_by_key(|a| a % 2);
    pp!(@it my_vec.iter());
}
cp::main!();
