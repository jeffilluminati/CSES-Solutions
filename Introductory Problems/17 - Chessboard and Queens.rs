fn dfs(
    row: usize,
    board: &[Vec<char>; 8],
    col_used: &mut [bool; 8],
    diag_down_used: &mut [bool; 15],
    diag_up_used: &mut [bool; 15],
) -> usize {
    if row == 8 {
        return 1;
    }

    let mut ways = 0;
    for col in 0..8 {
        if board[row][col] == '*' {
            continue;
        }

        let diag_down = row + col;
        let diag_up = row + 7 - col;
        if col_used[col] || diag_down_used[diag_down] || diag_up_used[diag_up] {
            continue;
        }

        col_used[col] = true;
        diag_down_used[diag_down] = true;
        diag_up_used[diag_up] = true;

        ways += dfs(row + 1, board, col_used, diag_down_used, diag_up_used);

        col_used[col] = false;
        diag_down_used[diag_down] = false;
        diag_up_used[diag_up] = false;
    }
    ways
}

pub fn solve() {
    cp::prepare!();
    sc!(board: [Chars; const 8]);

    let ans = dfs(
        0,
        &board,
        &mut [false; 8],
        &mut [false; 15],
        &mut [false; 15],
    );
    pp!(ans);
}

cp::main!();
