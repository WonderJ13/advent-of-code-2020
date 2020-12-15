use std::io;
use std::io::BufRead;
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut jolts: Vec<u64> = vec![];
    stdin.lines().for_each(|line| {
        let num = u64::from_str(&line.unwrap()).unwrap();
        jolts.push(num);
    });

    jolts.sort_unstable();

    let mut ones: u64 = 0;
    let mut threes: u64 = 1; //Device is always 3 higher than the highest joltage
    jolts.iter().fold(0, |prev_val, val| {
        let diff = val - prev_val;
        match diff {
            1 => ones += 1,
            3 => threes += 1,
            _ => (),
        };
        *val
    });
    println!("{}", ones * threes);
}
