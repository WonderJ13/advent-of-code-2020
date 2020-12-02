use std::io;
use std::io::BufRead;
use std::str::FromStr;

fn find_2020_sum(nums: &mut Vec<i64>) -> i64 {
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            for k in j+1..nums.len() {
                if nums[i] + nums[j] + nums[k] == 2020 {
                    return nums[i] * nums[j] * nums[k];
                }
            }
        }
    }
    return -1;
}

fn main() {
    let mut nums = Vec::new();
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    for line in stdin_lock.lines() {
        match i64::from_str(&line.unwrap()) {
            Ok(n) => nums.push(n),
            Err(_) => continue,
        }
    }

    println!("{}", find_2020_sum(&mut nums));
}
