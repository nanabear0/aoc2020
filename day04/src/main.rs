use regex::Regex;
use std::{collections::HashMap, fs::File, str::FromStr};
use std::{
    io::{prelude::*, BufReader},
    vec,
};

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl FromStr for Passport {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mandatory_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        let re = Regex::new(r"(\S+):(\S+)").unwrap();
        let mut matches: HashMap<String, String> = re
            .captures_iter(s)
            .map(|s| (s[1].trim().to_string(), s[2].trim().to_string()))
            .collect();
        if !mandatory_fields.iter().any(|f| !matches.contains_key(*f)) {
            Ok(Passport {
                byr: matches.remove("byr"),
                iyr: matches.remove("iyr"),
                eyr: matches.remove("eyr"),
                hgt: matches.remove("hgt"),
                hcl: matches.remove("hcl"),
                ecl: matches.remove("ecl"),
                pid: matches.remove("pid"),
                cid: matches.remove("cid"),
            })
        } else {
            Err("missing fields".to_owned())
        }
    }
}

impl Passport {
    fn check(&self) -> bool {
        self.check_byr()
            && self.check_iyr()
            && self.check_eyr()
            && self.check_hgt()
            && self.check_hcl()
            && self.check_ecl()
            && self.check_pid()
            && self.check_cid()
    }

    fn check_byr(&self) -> bool {
        if let Some(byr) = self.byr.as_ref() {
            byr.parse::<usize>()
                .map_or(false, |v| v >= 1920 && v <= 2002)
        } else {
            false
        }
    }
    fn check_iyr(&self) -> bool {
        if let Some(iyr) = self.iyr.as_ref() {
            iyr.parse::<usize>()
                .map_or(false, |v| v >= 2010 && v <= 2020)
        } else {
            false
        }
    }
    fn check_eyr(&self) -> bool {
        if let Some(eyr) = self.eyr.as_ref() {
            eyr.parse::<usize>()
                .map_or(false, |v| v >= 2020 && v <= 2030)
        } else {
            false
        }
    }
    fn check_hgt(&self) -> bool {
        if let Some(hgt) = self.hgt.as_ref() {
            let re = Regex::new(r"^(\d{2,3})(cm|in)").unwrap();
            if let Some(matches) = re.captures(hgt) {
                let c = &matches[2];
                let n = matches[1].parse::<usize>().unwrap_or(0);
                match c {
                    "cm" => n >= 150 && n <= 193,
                    "in" => n >= 59 && n <= 76,
                    _ => false,
                }
            } else {
                false
            }
        } else {
            false
        }
    }
    fn check_hcl(&self) -> bool {
        if let Some(hcl) = self.hcl.as_ref() {
            let re = Regex::new(r"^\#[0-9a-f]{6}$").unwrap();
            re.is_match(hcl)
        } else {
            false
        }
    }
    fn check_ecl(&self) -> bool {
        if let Some(ecl) = self.ecl.as_ref() {
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .iter()
                .any(|c| c == ecl)
        } else {
            false
        }
    }
    fn check_pid(&self) -> bool {
        if let Some(pid) = self.pid.as_ref() {
            let re = Regex::new(r"^\d{9}$").unwrap();
            re.is_match(pid)
        } else {
            false
        }
    }
    fn check_cid(&self) -> bool {
        true
    }
}

fn main() {
    let mut reader = BufReader::new(File::open("input.txt").unwrap());
    let mut v = String::new();
    reader.read_to_string(&mut v).unwrap();
    let passports: Vec<Passport> = v.split("\r\n\r\n").filter_map(|x| x.parse().ok()).collect();
    println!("part1: {:?}", passports.len());
    println!(
        "part2: {:?}",
        passports.iter().filter(|p| p.check()).count()
    );
}
