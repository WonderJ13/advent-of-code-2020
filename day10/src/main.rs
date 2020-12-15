use std::io;
use std::io::BufRead;
use std::str::FromStr;

fn add_prev_3(nums: &Vec<u64>, index: usize) -> u64 {
    match index {
        0 => 1,
        1 => nums[0],
        2 => nums[1] + nums[0],
        _ => nums[index-1] + nums[index-2] + nums[index-3],
    }
}

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut jolts: Vec<u64> = vec![];
    stdin.lines().for_each(|line| {
        let num = u64::from_str(&line.unwrap()).unwrap();
        jolts.push(num);
    });
    jolts.push(0);
    jolts.sort_unstable();
    jolts.push(jolts[jolts.len()-1] + 3);

    let jolt_max = jolts[jolts.len()-1] as usize + 1;
    let mut arrangements = vec![0; jolt_max];
    jolts.iter().for_each(|jolt| {
        let jolt = *jolt as usize;
        arrangements[jolt] = add_prev_3(&arrangements, jolt);
    });

    println!("{}", arrangements[jolt_max-1]);
}
