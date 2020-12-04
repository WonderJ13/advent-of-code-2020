use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
}

fn validate_passport(p: Passport) -> bool {
    return p.byr != "" && p.iyr != "" && p.eyr != "" && p.hgt != "" &&
            p.hcl != "" && p.ecl != "" && p.pid != ""
}

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut count = 0;

    let mut passport = Passport{
        byr: String::from(""), 
        iyr: String::from(""), 
        eyr: String::from(""), 
        hgt: String::from(""), 
        hcl: String::from(""), 
        ecl: String::from(""), 
        pid: String::from(""), 
        cid: String::from(""), 
    };
    for line in stdin.lines() {
        let line = line.unwrap();
        if line == "" {
            if validate_passport(passport) {
                count += 1;
            }
            passport = Passport{
                byr: String::from(""), 
                iyr: String::from(""), 
                eyr: String::from(""), 
                hgt: String::from(""), 
                hcl: String::from(""), 
                ecl: String::from(""), 
                pid: String::from(""), 
                cid: String::from(""), 
            };
            continue;
        }

        let fields: Vec<&str> = line.split(" ").collect();
        for field in fields {
            let foo = field.split(":").collect::<Vec<&str>>();
            let field = foo[0];
            let value = foo[1];
            match field {
                "byr" => passport.byr = String::from(value),
                "iyr" => passport.iyr = String::from(value),
                "eyr" => passport.eyr = String::from(value),
                "hgt" => passport.hgt = String::from(value),
                "hcl" => passport.hcl = String::from(value),
                "ecl" => passport.ecl = String::from(value),
                "pid" => passport.pid = String::from(value),
                "cid" => passport.cid = String::from(value),
                _ => println!("Found something weird: {}: {}", field, value),
            }
        }
    }
    println!("{}", count);
}
