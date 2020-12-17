use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use std::str::FromStr;

struct MemLocIter {
    x_pos: u64,
    counter: u64,
    one_bits: u8,
}

impl MemLocIter {
    fn from(x_pos: u64) -> MemLocIter {
        let mut one_bits: u8 = 0;
        for i in 0..64 {
            if x_pos & 1 << i > 0 {
                one_bits += 1;
            }
        }

        MemLocIter{
            x_pos: x_pos,
            counter: 0,
            one_bits: one_bits,
        }
    }
}

impl Iterator for MemLocIter {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        if 2_u64.pow(self.one_bits.into()) == self.counter {
            return None
        }

        let mut ret = 0;
        let mut x_pos_mask = 1;
        for i in 0..self.one_bits {
            let counter_mask = 1 << i;
            
            loop {
                if self.x_pos & x_pos_mask > 0 {
                    if self.counter & counter_mask > 0 {
                        ret |= x_pos_mask;
                    }
                    x_pos_mask <<= 1;
                    break;
                }
                x_pos_mask <<= 1;
            }
        }
        self.counter += 1;
        Some(ret)
    }
}

#[derive(Debug)]
struct State {
    mem: HashMap<u64, u64>,
    x_pos: u64,
    hcv: u64,
}

impl State {
    fn new() -> State {
        State{
            mem: HashMap::new(),
            x_pos: 0,
            hcv: 0,
        }
    }

    fn set_mask(&mut self, mask: &str) {
        let mut x_pos = 0;
        let mut hcv = 0;

        let mask = String::from(mask);
        mask.chars().for_each(|c| match c {
            'X' => {
                x_pos = (x_pos << 1) + 1;
                hcv <<= 1;
            },
            '1' => {
                x_pos <<= 1;
                hcv = (hcv << 1) + 1;
            },
            '0' => {
                x_pos <<= 1;
                hcv <<= 1;
            },
            _ => (),
        });

        self.x_pos = x_pos;
        self.hcv = hcv;
    }

    fn set_memory(&mut self, mem_loc: u64, val: u64) {
        let base_mem_loc = (mem_loc | self.hcv | self.x_pos) ^ self.x_pos;
        let mem_loc_iter = MemLocIter::from(self.x_pos);

        mem_loc_iter.for_each(|mask| {
            self.mem.insert(base_mem_loc | mask, val);
        });
    }

    fn sum_of_memory(&self) -> u64 {
        let mut sum = 0;
        self.mem.iter().for_each(|(_, val)| {
            sum += val;
        });
        sum
    }
}

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut state = State::new();
    for line in stdin.lines() {
        let line = line.unwrap();

        //First 4 characters are always either "mask" or "mem[".
        //We can determine what to do with this
        match &line[0..4] {
            "mask" => {
                //"mask = " == 7 characters
                let mask = &line[7..];
                state.set_mask(mask);
            },
            "mem[" => {
                let back_bracket_index = line.find(']').unwrap();
                let mem_loc = u64::from_str(
                    &line[4..back_bracket_index]
                ).unwrap();
                let val = u64::from_str(
                    &line[back_bracket_index+4..]
                ).unwrap();
                state.set_memory(mem_loc, val);
            },
            _ => {
                println!("UNREACHABLE");
            },
        }
    }

    println!("{}", state.sum_of_memory());
}
