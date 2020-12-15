use std::io;
use std::io::BufRead;
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut dir = 0;
    let mut x = 0;
    let mut y = 0;
    for line in stdin.lines() {
        let line = line.unwrap();
        let mut line = line.chars();
        let command = line.next().unwrap();
        let num = i64::from_str(&line.as_str()).unwrap();
        match command {
            'N' => y += num,
            'S' => y -= num,
            'E' => x += num,
            'W' => x -= num,
            'L' => dir = (dir + num) % 360,
            'R' => dir = (dir + (360 - num)) % 360,
            'F' => {
                match dir {
                    0 => x += num,
                    90 => y += num,
                    180 => x -= num,
                    270 => y -= num,
                    _ => (),
                }
            },
            _ => (),
        };
    }
    println!("{}", x.abs() + y.abs());
}
