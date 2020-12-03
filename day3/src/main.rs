use std::io;
use std::io::BufRead;

fn count_trees_1down_3right(board: Vec<Vec<bool>>) -> i32 {
    let rows = board.len();
    let cols = board[0].len();

    let (mut y, mut x) = (0, 0);
    let mut count = 0;
    while y < rows {
        if board[y][x] {
            count += 1;
        }
        y += 1;
        x = (x + 3) % cols;
    }
    count
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
    println!("{}", count_trees_1down_3right(board));
}
