use std::io;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    ACC(i64),
    NOP(i64),
    JMP(i64),
}

use Instruction::{ACC, NOP, JMP};

fn run_code(ins: Vec<Instruction>) -> i64 {
    let mut ins_visited: Vec<bool> = vec![false; ins.len()];

    let mut pc: i64 = 0;
    let mut acc: i64 = 0;
    loop {
        let pci = pc as usize;
        //Have we run this line?
        if ins_visited[pci] {
            return acc;
        }
        ins_visited[pci] = true;

        match ins[pci] {
            ACC(n) => {
                acc += n;
                pc += 1;
            },
            NOP(_) => {
                pc += 1;
            },
            JMP(n) => {
                pc += n;
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut ins: Vec<Instruction> = vec![];
    for line in stdin.lines() {
        let line = line.unwrap();
        let ins_parts: Vec<&str> = line.split(" ").collect();
        let num = i64::from_str(&ins_parts[1]).unwrap();
        
        let instruction = match ins_parts[0] {
            "acc" => ACC(num),
            "nop" => NOP(num),
            "jmp" => JMP(num),
            &_ => panic!("Unreachable"),
        };
        ins.push(instruction);
    }

    let ins_test: Vec<Instruction> = vec![
        NOP(0),
        ACC(1),
        JMP(4),
        ACC(3),
        JMP(-3),
        ACC(-99),
        ACC(1),
        JMP(-4),
        ACC(6),
    ];

    println!("{}", run_code(ins));
}
