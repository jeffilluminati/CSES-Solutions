const BASE_FIX: [[u16; 4]; 4] = [[0, 2, 0, 0], [2, 2, 0, 0], [0, 0, 2, 0], [0, 0, 0, 0]];
const N4_FIX: [[u16; 4]; 4] = [[0, 0, 0, 2], [0, 0, 0, 0], [0, 0, 0, 0], [2, 0, 0, 0]];

pub fn solve() {
    cp::prepare!();
    sc!(n: u16);

    for i in 0..n {
        let half_i = (i + 1) >> 1;

        let mut third = (i + 2) / 3;
        let mut rem3 = (i + 2) % 3;

        for j in 0..n {
            let half_j = (j + 1) >> 1;
            let mut d = half_i.max(half_j).max(third);
            d += (d + i + j) & 1;

            if i < 4 && j < 4 {
                d += BASE_FIX[i as usize][j as usize];
                if n == 4 {
                    d += N4_FIX[i as usize][j as usize];
                }
            }

            if j + 1 != n {
                pp!(d, ' ', !);
            } else {
                pp!(d);
            }

            if rem3 == 2 {
                rem3 = 0;
                third += 1;
            } else {
                rem3 += 1;
            }
        }
    }
}

cp::main!();
