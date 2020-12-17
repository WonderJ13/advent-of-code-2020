use std::io;

fn main() {
    let stdin = io::stdin();

    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    let mut nums: Vec<usize> = buffer
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    for i in nums.len()..2020 {
        let num = nums.pop().unwrap();
        let insert = match nums.iter().rposition(|&n| n == num) {
            Some(n) => i - n - 1,
            None => 0,
        };
        nums.push(num);
        nums.push(insert);
    }

    println!("{}", nums.last().unwrap());
}
