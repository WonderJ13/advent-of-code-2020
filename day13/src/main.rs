use std::io;
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    stdin.read_line(&mut buffer).unwrap();
    let earliest_time = u64::from_str(&buffer.trim()).unwrap();
    buffer.clear();

    stdin.read_line(&mut buffer).unwrap();
    let mut bus_data: Vec<(u64, u64)> = buffer
        .trim()
        .split(',')
        .map(|x| match u64::from_str(&x) {
                Ok(n) => n,
                Err(_) => 0,
            }
        )
        .filter(|x| *x != 0)
        .map(|x| (x, x - (earliest_time % x)))
        .collect();

    bus_data.sort_by_key(|x| x.1);

    println!("{}", bus_data[0].0 * bus_data[0].1);
}
