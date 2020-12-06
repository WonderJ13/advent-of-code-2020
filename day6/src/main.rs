use std::io;
use std::io::BufRead;

fn count_group(answers: Vec<String>) -> u8 {
    let mut answer_sheet: [bool; 26] = [false; 26];
    for answer in answers {
        for a in answer.chars() {
            let i = (a as usize) - ('a' as usize);
            answer_sheet[i] = true;
        }
    }

    let mut count = 0;
    for a in answer_sheet.iter() {
        count += *a as u8;
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
