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

fn step_through_code(ins: &Vec<Instruction>, acc: i64, pc: i64) -> (i64, i64) {
    return match ins[pc as usize] {
        ACC(n) => (acc + n, pc + 1),
        NOP(_) => (acc, pc + 1),
        JMP(n) => (acc, pc + n),
    };
}

fn get_potential_broken_lines(ins: &Vec<Instruction>) -> Vec<usize> {
    let mut ins_visited: Vec<bool> = vec![false; ins.len()];
    let mut pot_ins: Vec<usize> = vec![];

    let mut acc: i64 = 0;
    let mut pc: i64 = 0;
    loop {
        let pci = pc as usize;

        //Are we at the end of the program?
        if ins_visited[pci] {
            return pot_ins;
        }
        ins_visited[pci] = true;

        //List suspect lines
        match ins[pci] {
            ACC(_) => (),
            NOP(_) => pot_ins.push(pci),
            JMP(_) => pot_ins.push(pci),
        };

        let (acc_n, pc_n) = step_through_code(&ins, acc, pc);
        acc = acc_n;
        pc = pc_n;
    }
}

fn run_code(ins: &Vec<Instruction>) -> Result<i64, i64> {
    let mut ins_visited: Vec<bool> = vec![false; ins.len()];

    let mut acc: i64 = 0;
    let mut pc: i64 = 0;
    loop {
        let pci = pc as usize;

        //Are we at the end of the program?
        if pci >= ins.len() {
            return Ok(acc);
        }

        //Have we run this line?
        if ins_visited[pci] {
            return Err(acc);
        }
        ins_visited[pci] = true;

        let (acc_n, pc_n) = step_through_code(&ins, acc, pc);
        acc = acc_n;
        pc = pc_n;
    }
}

fn flip_ins(ins: &Vec<Instruction>, pc: usize) -> Instruction {
    return match ins[pc] {
        ACC(n) => ACC(n),
        NOP(n) => JMP(n),
        JMP(n) => NOP(n),
    };
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

    for pc in get_potential_broken_lines(&ins) {
        ins[pc] = flip_ins(&ins, pc);

        match run_code(&ins) {
            Ok(n) => {
                println!("{}", n);
                break;
            },
            Err(_) => (),
        }

        ins[pc] = flip_ins(&ins, pc);
    }
}
