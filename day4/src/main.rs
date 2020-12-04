use std::io;
use std::io::BufRead;
use std::str::FromStr;

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
    //BYR
    let byr = i32::from_str(p.byr.as_str());
    let byr = match byr {
        Ok(b) => b,
        Err(_) => 0,
    };
    if byr < 1920 || byr > 2002 {
        return false;
    }

    //IYR
    let iyr = i32::from_str(p.iyr.as_str());
    let iyr = match iyr {
        Ok(i) => i,
        Err(_) => 0,
    };
    if iyr < 2010 || iyr > 2020 {
        return false;
    }

    //EYR
    let eyr = i32::from_str(p.eyr.as_str());
    let eyr = match eyr {
        Ok(e) => e,
        Err(_) => 0,
    };
    if eyr < 2020 || eyr > 2030 {
        return false;
    }

    //HGT
    if p.hgt.ends_with("in") {
        let hgt = i32::from_str(p.hgt.strip_suffix("in").unwrap());
        let hgt = match hgt {
            Ok(h) => h,
            Err(_) => 0,
        };
        if hgt < 59 || hgt > 76 {
            return false;
        }
    } else if p.hgt.ends_with("cm") {
        let hgt = i32::from_str(p.hgt.strip_suffix("cm").unwrap());
        let hgt = match hgt {
            Ok(h) => h,
            Err(_) => 0,
        };
        if hgt < 150 || hgt > 193 {
            return false;
        }
    } else {
        return false;
    }

    //HCL
    if p.hcl.len() != 7 || !p.hcl.starts_with("#") {
        return false;
    }
    let hcl = i32::from_str_radix(p.hcl.strip_prefix("#").unwrap(), 16);
    let hcl = match hcl {
        Ok(h) => h,
        Err(_) => -1,
    };
    if hcl < 0 || hcl > 0xFFFFFF {
        return false;
    }

    //ECL
    let ecl = match p.ecl.as_str() {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    };
    if !ecl {
        return false;
    }

    //PID
    if p.pid.len() != 9 {
        return false;
    }
    let pid = i32::from_str(p.pid.as_str());
    let pid = match pid {
        Ok(p) => p,
        Err(_) => -1,
    };
    if pid < 0 || pid > 999_999_999 {
        return false;
    }

    return true;
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
