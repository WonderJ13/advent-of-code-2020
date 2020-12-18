use std::io;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();

    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    let nums: Vec<usize> = buffer
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut nums_map: HashMap<usize, usize> = HashMap::new();
    nums.iter()
        .take(nums.len() - 1)
        .enumerate()
        .for_each(|(i, &n)| {
            nums_map.insert(n, i+1);
        });

    let mut num_to_search = *nums.last().unwrap();
    for i in nums.len()..30_000_000 {
        let insert = match nums_map.get(&num_to_search) {
            Some(n) => i - n,
            None => 0,
        };
        nums_map.insert(num_to_search, i);
        num_to_search = insert;
    }

    println!("{}", num_to_search);
}
