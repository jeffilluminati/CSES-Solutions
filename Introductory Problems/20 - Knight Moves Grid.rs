const DX: [i16; 8] = [1, 2, 2, 1, -1, -2, -2, -1];
const DY: [i16; 8] = [2, 1, -1, -2, -2, -1, 1, 2];

pub fn solve() {
    cp::prepare!();
    let mut out = cp::tools::FastOutput::stdout();

    use std::collections::VecDeque;
    sc!(n: u16);

    let mut queue = VecDeque::<(i16, i16)>::new();
    let mut grid = [[0i16; 1001]; 1001];

    queue.push_back((0, 0));
    grid[0][0] = 1;
    while queue.len() > 0 {
        let (x, y) = queue.pop_front().unwrap();

        (0..8).for_each(|i| {
            let (vx, vy) = (x + DX[i], y + DY[i]);

            if vx >= 0
                && vy >= 0
                && vy < n as i16
                && vx < n as i16
                && grid[vx as usize][vy as usize] == 0
            {
                grid[vx as usize][vy as usize] = grid[x as usize][y as usize] + 1;
                queue.push_back((vx, vy));
            }
        });
    }

    for i in 0..n as usize {
        for j in 0..n as usize {
            out.u16((grid[i][j] - 1) as u16);
            out.byte(b' ');
        }
        out.byte(b'\n');
    }

    out.flush();
}

cp::main!();
