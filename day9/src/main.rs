use std::io;
use std::io::BufRead;
use std::str::FromStr;

fn validate_num(preamble: &Vec<u64>, num: u64) -> bool {
    for i in 0..preamble.len() {
        for j in i+1..preamble.len() {
            if preamble[i] + preamble[j] == num {
                return true;
            }
        }
    }
    return false;
}

fn find_contiguous_set(haystack: Vec<u64>, needle: u64) -> Vec<u64> {
    let mut set: Vec<u64> = vec![];
    for i in 0..haystack.len() {
        for j in i..haystack.len() {
            set.push(haystack[j]);
            let num = set.iter().fold(0, |acc, val| {
                acc + val
            });

            if num > needle {
                break;
            }
            if num == needle {
                return set
            }
        }
        set = vec![];
    }
    set
}

const PREAMBLE_LEN: usize = 25;

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut preamble: Vec<u64> = vec![];
    let mut nums: Vec<u64> = vec![];
    for line in stdin.lines() {
        let num = u64::from_str(&line.unwrap()).unwrap();
        if preamble.len() != PREAMBLE_LEN {
            preamble.push(num);
        } else {
            nums.push(num);
        }
    }

    let mut used_numbers: Vec<u64> = vec![];
    let mut invalid_num: u64 = 0;
    for num in nums {
        if !validate_num(&preamble, num) {
            invalid_num = num;
            break;
        }
        used_numbers.push(preamble.remove(0));
        preamble.push(num);
    }

    used_numbers.append(&mut preamble);
    let mut set = find_contiguous_set(used_numbers, invalid_num);
    set.sort_unstable();
    println!("{}", set[0] + set[set.len()-1]);
}
