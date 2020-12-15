use std::io;
use std::io::BufRead;
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut ship_x = 0;
    let mut ship_y = 0;
    let mut wayp_x = 10;
    let mut wayp_y = 1;
    for line in stdin.lines() {
        let line = line.unwrap();
        let mut line = line.chars();
        let command = line.next().unwrap();
        let num = i64::from_str(&line.as_str()).unwrap();
        match command {
            'N' => wayp_y += num,
            'S' => wayp_y -= num,
            'E' => wayp_x += num,
            'W' => wayp_x -= num,
            'L' => {
                match num {
                    90 => {
                        let tmp = wayp_y;
                        wayp_y = wayp_x;
                        wayp_x = -tmp;
                    },
                    180 => {
                        wayp_y = -wayp_y;
                        wayp_x = -wayp_x;
                    },
                    270 => {
                        let tmp = wayp_y;
                        wayp_y = -wayp_x;
                        wayp_x = tmp;
                    },
                    _ => (),
                }
            }
            'R' => {
                match num {
                    90 => {
                        let tmp = wayp_y;
                        wayp_y = -wayp_x;
                        wayp_x = tmp;
                    },
                    180 => {
                        wayp_y = -wayp_y;
                        wayp_x = -wayp_x;
                    },
                    270 => {
                        let tmp = wayp_y;
                        wayp_y = wayp_x;
                        wayp_x = -tmp;
                    },
                    _ => (),
                }
            },
            'F' => {
                ship_x += wayp_x * num;
                ship_y += wayp_y * num;
            },
            _ => (),
        };
    }
    println!("{}", ship_x.abs() + ship_y.abs());
}
