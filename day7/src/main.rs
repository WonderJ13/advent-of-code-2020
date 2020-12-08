use std::io;
use std::io::BufRead;
use std::str::FromStr;

fn bag_contains_shiny_gold(bags: &Vec<(String, Vec<(u64, String)>)>, bag_name: &String) -> bool {
    //Get index of bag we're looking at
    let i = bags.binary_search_by(|bag| {
        bag_name.cmp(&bag.0)
    }).unwrap();

    for sub_bag in &bags[i].1 {
        if sub_bag.1 == String::from("shiny gold") || bag_contains_shiny_gold(bags, &sub_bag.1) {
            return true;
        }
    }
    return false;
}

fn count_contains_shiny_gold(bags: Vec<(String, Vec<(u64, String)>)>) -> u64 {
    let mut count: u64 = 0;
    bags.iter().for_each(|bag| {
        if bag_contains_shiny_gold(&bags, &bag.0) {
            count += 1;
        }
    });
    count
}

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut bags: Vec<(String, Vec<(u64, String)>)> = vec![];

    for rule in stdin.lines() {
        let rule = rule.unwrap();
        let bag_relationship: Vec<&str> = rule.split(" bags contain ").collect();
        let bag_name = bag_relationship[0].to_string();

        let mut sub_bags: Vec<(u64, String)> = vec![];
        let sub_bag_listings: Vec<&str> = bag_relationship[1].split(", ").collect();
        for sub_bag_rule in sub_bag_listings {
            let mut words: Vec<&str> = sub_bag_rule.split(" ").collect();
            let amt = match u64::from_str(&words[0]) {
                Ok(n) => n,
                Err(_) => break, //Only occurs on "no other bags"
            };

            //Create sub_bag_name
            words.remove(0); //Remove amt
            words.remove(words.len() - 1); //Remove last word ("bag[s][.,]")
            let sub_bag_name = words.join(" ");
            let sub_bag_name = String::from(sub_bag_name);

            sub_bags.push((amt, sub_bag_name));
        }
        let index = bags.binary_search_by(|bag| {
            bag_name.cmp(&bag.0)
        });
        let index = match index {
            Ok(_) => 0,
            Err(n) => n
        };
        bags.insert(index, (bag_name, sub_bags));
    }

    let bags_test = vec![
        (String::from("light red"), vec![(1, String::from("bright white")), (2, String::from("muted yellow"))]),
        (String::from("dark orange"), vec![(3, String::from("bright white")), (4, String::from("muted yellow"))]),
        (String::from("bright white"), vec![(1, String::from("shiny gold"))]),
        (String::from("muted yellow"), vec![(2, String::from("shiny gold")), (9, String::from("faded blue"))]),
        (String::from("shiny gold"), vec![(1, String::from("dark olive")), (2, String::from("vibrant plum"))]),
        (String::from("dark olive"), vec![(3, String::from("faded blue")), (4, String::from("dotted black"))]),
        (String::from("vibrant plum"), vec![(5, String::from("faded blue")), (6, String::from("dotted black"))]),
        (String::from("faded blue"), vec![]),
        (String::from("dotted black"), vec![]),
    ];

    println!("{:?}", count_contains_shiny_gold(bags));
}
