use std::io;
use std::str::FromStr;

enum Bus {
    ID(u64),
    NIL,
}

use Bus::{ID, NIL};

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    //Skip first line
    stdin.read_line(&mut buffer).unwrap();
    buffer.clear();

    stdin.read_line(&mut buffer).unwrap();
    let bus_ids: Vec<(usize, Bus)> = buffer
        .trim()
        .split(',')
        .map(|x| match u64::from_str(&x) {
                Ok(n) => ID(n),
                Err(_) => NIL,
            }
        )
        .enumerate()
        .collect();

    let mut time = match bus_ids[0].1 {
        ID(n) => n,
        NIL => 0,
    };

    let mut jump_by = 1;
    for (i, bus) in bus_ids {
        let bus_id = match bus {
            ID(n) => n,
            NIL => continue,
        };

        loop {
            if (time + i as u64) % bus_id == 0 {
                jump_by = jump_by * bus_id;
                break;
            }
            time += jump_by;
        }
    }

    println!("{:?}", time);
}
