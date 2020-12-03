use std::io;
use std::io::BufRead;

fn count_trees(board: &Vec<Vec<bool>>, slope_y: usize, slope_x: usize) -> i64 {
    let rows = board.len();
    let cols = board[0].len();

    let (mut y, mut x) = (0, 0);
    let mut count = 0;
    while y < rows {
        if board[y][x] {
            count += 1;
        }
        y += slope_y;
        x = (x + slope_x) % cols;
    }
    count
}

fn multiply_trees(board: Vec<Vec<bool>>) -> i64 {
    count_trees(&board, 1, 1) *
    count_trees(&board, 1, 3) *
    count_trees(&board, 1, 5) *
    count_trees(&board, 1, 7) *
    count_trees(&board, 2, 1)
}

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut board: Vec<Vec<bool>> = Vec::new();

    //Read board state
    for line in stdin.lines() {
        let line = line.unwrap();
        let mut row = Vec::new();
        line.chars().for_each(|c| {
            row.push(c == '#');
        });
        board.push(row);
    }

    //Perform calculations
    println!("{}", multiply_trees(board));
}
