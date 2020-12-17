use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use std::str::FromStr;

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
        let val = (val & self.x_pos) | self.hcv;
        self.mem.insert(mem_loc, val);
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
