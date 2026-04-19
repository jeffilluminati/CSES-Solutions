pub fn solve() {
    cp::prepare!();
    sc!(n: usize);

    let mut grid = vec![vec![0u8; n]; n];

    for i in 0..n {
        for j in 0..n {
            if i == 0 && j == 0 {
                grid[i][j] = 0;
            } else if i == 0 {
                grid[i][j] = j as u8;
            } else if j == 0 {
                grid[i][j] = i as u8;
            } else {
                let mut m = [false; 200];
                (0..i)
                    .map(|x| grid[x][j])
                    .chain((0..j).map(|x| grid[i][x]))
                    .for_each(|i| m[i as usize] = true);

                grid[i][j] = m.iter().position(|b| !b).unwrap() as u8;
            }
        }
    }

    grid.iter().for_each(|l| pp!(@it l.into_iter()));
}

cp::main!();
