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

    let mut id_filled: [bool; 1024] = [false; 1024];
    for line in stdin.lines() {
        id_filled[id_from_pass(line.unwrap()) as usize] = true
    }
    //first viable seat has to be id 9
    //last viable seat has to be id 1015
    for i in 9..1015 {
        if !id_filled[i] {
            println!("{}", i);
            break;
        }
    }
}
