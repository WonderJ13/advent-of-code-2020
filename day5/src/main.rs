use std::io;
use std::io::BufRead;

fn id_from_pass(pass: String) -> i32 {
    let mut id: i32 = 0;
    pass.chars().for_each(|val| {
        id <<= 1;
        id |= if val == 'B' || val == 'R' { 1 } else { 0 };
    });
    id
}

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut max_id: i32 = -1;
    for line in stdin.lines() {
        let id = id_from_pass(line.unwrap());
        if max_id < id {
            max_id = id;
        }
    }
    println!("{}", max_id);
}
