use std::io;
use std::io::BufRead;

fn count_group(passenger_answers: Vec<String>) -> u8 {
    let mut answer_sheet: [u64; 26] = [0; 26];
    let passenger_count: u64 = passenger_answers.len() as u64;
    for passenger in passenger_answers {
        for a in passenger.chars() {
            let i = (a as usize) - ('a' as usize);
            answer_sheet[i] += 1;
        }
    }

    let mut count = 0;
    for answer_count in answer_sheet.iter() {
        count += if passenger_count == *answer_count { 1 } else { 0 };
    }
    count
}

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut count: u64 = 0;
    let mut answers = vec![];
    for line in stdin.lines() {
        let line = line.unwrap();
        if line == "" {
            count += count_group(answers) as u64;
            answers = vec![];
            continue;
        }
        answers.push(line);
    }

    println!("{}", count);
}
